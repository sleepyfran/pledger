pub mod payee_description_section;
pub mod posting;

use nom::combinator::map;
use nom::error::context;
use nom::sequence::tuple;
use nom::IResult;

use super::ast::{PayeeSectionType, Transaction};
use super::common::date;

pub fn parse(input: &str) -> IResult<&str, Transaction> {
    context(
        "transaction",
        map(
            tuple((
                date::parse,
                payee_description_section::parse,
                posting::parse_multiple,
            )),
            |(date, payee_description_section, postings)| {
                let (payee, description) = match payee_description_section {
                    PayeeSectionType::Empty => ("".to_owned(), "".to_owned()),
                    PayeeSectionType::PayeeOnly(payee) => (payee, "".to_owned()),
                    PayeeSectionType::PayeeAndDescription((payee, description)) => {
                        (payee, description)
                    }
                };

                Transaction {
                    date,
                    payee,
                    description,
                    postings,
                }
            },
        ),
    )(input)
}

#[cfg(test)]
mod test {}
