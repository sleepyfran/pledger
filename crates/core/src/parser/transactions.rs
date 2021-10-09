use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending, multispace0, space0};
use nom::combinator::{map, opt};
use nom::multi::many_till;
use nom::sequence::{delimited, preceded, tuple};
use nom::{error::context, IResult};

use super::ast::{Description, Payee, Transaction};
use super::common::date;

enum PayeeSectionType {
    PayeeOnly(Payee),
    PayeeAndDescription((Payee, Description)),
}

pub fn parse(input: &str) -> IResult<&str, Transaction> {
    context(
        "transaction",
        map(
            tuple((date::parse, opt(parse_payee_description_section))),
            |(date, payee_section)| {
                let (payee, description) = match payee_section {
                    Some(section) => match section {
                        PayeeSectionType::PayeeOnly(payee) => (payee, "".to_owned()),
                        PayeeSectionType::PayeeAndDescription((payee, description)) => {
                            (payee, description)
                        }
                    },
                    None => ("".to_owned(), "".to_owned()),
                };

                Transaction {
                    date,
                    payee,
                    description,
                    postings: Vec::new(),
                }
            },
        ),
    )(input)
}

fn parse_payee_description_section(input: &str) -> IResult<&str, PayeeSectionType> {
    context(
        "transaction payee section",
        preceded(
            multispace0,
            alt((
                map(parse_payee_only, PayeeSectionType::PayeeOnly),
                map(
                    parse_payee_with_description,
                    PayeeSectionType::PayeeAndDescription,
                ),
            )),
        ),
    )(input)
}

fn parse_payee_only(input: &str) -> IResult<&str, Payee> {
    context(
        "transaction payee",
        preceded(
            multispace0,
            map(
                many_till(alt((alphanumeric1, space0)), line_ending),
                |(elements, _)| elements.join(""),
            ),
        ),
    )(input)
}

fn parse_payee_with_description(input: &str) -> IResult<&str, (Payee, Description)> {
    context(
        "transaction payee with description",
        map(
            tuple((
                map(
                    many_till(
                        alt((alphanumeric1, space0)),
                        delimited(multispace0, tag("|"), multispace0),
                    ),
                    |(elements, _)| elements.join(""),
                ),
                map(
                    many_till(alt((alphanumeric1, space0)), line_ending),
                    |(elements, _)| elements.join(""),
                ),
            )),
            |(payee, description)| (payee, description),
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::parser::ast::Transaction;
    use chrono::{Date, NaiveDate, Utc};

    use super::parse;
    use crate::parser::ast::ParsedDate;

    fn test_date() -> ParsedDate {
        ParsedDate::Full(Date::<Utc>::from_utc(NaiveDate::from_ymd(2021, 10, 8), Utc))
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
            parse("2021-10-08 Test\n"),
            Ok((
                "",
                Transaction {
                    date: test_date(),
                    payee: "Test".to_owned(),
                    description: "".to_owned(),
                    postings: Vec::new()
                }
            ))
        );

        assert_eq!(
            parse("2021-10-08 Test with spaces\n"),
            Ok((
                "",
                Transaction {
                    date: test_date(),
                    payee: "Test with spaces".to_owned(),
                    description: "".to_owned(),
                    postings: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parses_valid_transaction_with_payee_and_description() {
        assert_eq!(
            parse("2021-10-08 Test | Test description\n"),
            Ok((
                "",
                Transaction {
                    date: test_date(),
                    payee: "Test".to_owned(),
                    description: "Test description".to_owned(),
                    postings: Vec::new()
                }
            ))
        );

        assert_eq!(
            parse("2021-10-08 Test with spaces | Test description\n"),
            Ok((
                "",
                Transaction {
                    date: test_date(),
                    payee: "Test with spaces".to_owned(),
                    description: "Test description".to_owned(),
                    postings: Vec::new()
                }
            ))
        );
    }
}
