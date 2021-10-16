pub mod payee_description_section;
pub mod posting;
pub mod status;
pub mod tags;

use nom::character::complete::{line_ending, space0};
use nom::combinator::{map, opt};
use nom::error::context;
use nom::sequence::{preceded, tuple};
use nom::IResult;

use super::ast::{PayeeSectionType, Transaction};
use super::common::date;

pub fn parse(input: &str) -> IResult<&str, Transaction> {
    context(
        "transaction",
        map(
            tuple((
                date::parse,
                status::parse,
                payee_description_section::parse,
                preceded(space0, opt(tags::parse)),
                line_ending,
                posting::parse_multiple,
            )),
            |(date, status, payee_description_section, tags, _, postings)| {
                let (payee, description) = match payee_description_section {
                    PayeeSectionType::Empty => ("".to_owned(), "".to_owned()),
                    PayeeSectionType::PayeeOnly(payee) => (payee, "".to_owned()),
                    PayeeSectionType::PayeeAndDescription((payee, description)) => {
                        (payee, description)
                    }
                };

                Transaction {
                    date,
                    status,
                    payee,
                    description,
                    tags: tags.unwrap_or_else(Vec::new),
                    postings,
                }
            },
        ),
    )(input)
}

#[cfg(test)]
mod test {}
