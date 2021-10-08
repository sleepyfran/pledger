use nom::branch::alt;
use nom::character::complete::{alphanumeric1, line_ending, multispace0, space0};
use nom::combinator::{map, opt};
use nom::multi::many_till;
use nom::sequence::{preceded, tuple};
use nom::{error::context, IResult};

use super::ast::Transaction;
use super::common::date;

pub fn parse(input: &str) -> IResult<&str, Transaction> {
    context(
        "transaction",
        map(tuple((date::parse, opt(parse_payee))), |(date, payee)| {
            Transaction {
                date: date,
                payee: payee.unwrap_or_default(),
                description: "".to_owned(),
                postings: Vec::new(),
            }
        }),
    )(input)
}

fn parse_payee(input: &str) -> IResult<&str, String> {
    context(
        "transaction payee",
        preceded(
            multispace0,
            map(
                many_till(alt((alphanumeric1, space0)), line_ending),
                |(elements, _)| elements.join(" "),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::parser::ast::Transaction;
    use chrono::{Date, NaiveDate, Utc};

    use super::parse;

    fn test_date() -> Date<Utc> {
        Date::<Utc>::from_utc(NaiveDate::from_ymd(2021, 10, 8), Utc)
    }

    #[test]
    fn parses_valid_transaction_without_payee_or_description() {
        assert_eq!(
            parse("2021-10-08\n"),
            Ok((
                "",
                Transaction {
                    date: test_date(),
                    payee: "".to_owned(),
                    description: "".to_owned(),
                    postings: Vec::new()
                }
            ))
        )
    }

    #[test]
    fn parses_valid_transaction_with_payee_and_no_description() {
        assert_eq!(
            parse("2021-10-08 test\n"),
            Ok((
                "",
                Transaction {
                    date: test_date(),
                    payee: "test".to_owned(),
                    description: "".to_owned(),
                    postings: Vec::new()
                }
            ))
        )
    }
}
