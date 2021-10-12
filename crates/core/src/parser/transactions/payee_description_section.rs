use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, multispace0, space0},
    combinator::map,
    error::context,
    multi::many_till,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::parser::ast::{Description, Payee, PayeeSectionType};

/// Parses the section that include a payee and a description separated by a vertical bar.
pub fn parse(input: &str) -> IResult<&str, PayeeSectionType> {
    context(
        "payee | description",
        preceded(
            space0,
            alt((
                map(line_ending, |_| PayeeSectionType::Empty),
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
            space0,
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
                        delimited(space0, tag("|"), multispace0),
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
    use super::{parse, PayeeSectionType};

    #[test]
    fn parses_empty_payee_and_description() {
        assert_eq!(parse("\n"), Ok(("", PayeeSectionType::Empty)))
    }

    #[test]
    fn parses_valid_input_with_payee_and_no_description() {
        assert_eq!(
            parse("Test\n"),
            Ok(("", PayeeSectionType::PayeeOnly("Test".to_owned())))
        );

        assert_eq!(
            parse("Test with spaces\n"),
            Ok((
                "",
                PayeeSectionType::PayeeOnly("Test with spaces".to_owned())
            ))
        );
    }

    #[test]
    fn parses_valid_transaction_with_payee_and_description() {
        assert_eq!(
            parse("Test | Test description\n"),
            Ok((
                "",
                PayeeSectionType::PayeeAndDescription((
                    "Test".to_owned(),
                    "Test description".to_owned()
                ))
            ))
        );

        assert_eq!(
            parse("Test with spaces | Test description\n"),
            Ok((
                "",
                PayeeSectionType::PayeeAndDescription((
                    "Test with spaces".to_owned(),
                    "Test description".to_owned()
                ))
            ))
        );
    }
}
