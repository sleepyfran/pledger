use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    combinator::{map, opt},
    error::{context, ContextError, ParseError},
    sequence::{preceded, tuple},
    IResult,
};

use crate::parser::ast::PayeeSectionType;

/// Parses the section that include a payee and a description separated by a vertical bar.
pub fn parse<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PayeeSectionType, E> {
    context(
        "payee | description",
        preceded(
            space0,
            map(
                tuple((
                    opt(spaced_alphanumeric1),
                    opt(tag("|")),
                    opt(spaced_alphanumeric1),
                )),
                |(payee, _, description)| match payee {
                    Some(payee) => match description {
                        Some(description) => PayeeSectionType::PayeeAndDescription((
                            payee.trim().to_owned(),
                            description.trim().to_owned(),
                        )),
                        None => PayeeSectionType::PayeeOnly(payee.trim().to_owned()),
                    },
                    None => PayeeSectionType::Empty,
                },
            ),
        ),
    )(input)
}

fn spaced_alphanumeric1<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, String, E> {
    context(
        "spaced alphanumeric",
        map(
            take_while1(|c: char| c.is_alphanumeric() || c == ' '),
            String::from,
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use nom::error::Error;

    use super::{parse, PayeeSectionType};

    #[test]
    fn parses_empty_payee_and_description() {
        assert_eq!(parse::<Error<&str>>(""), Ok(("", PayeeSectionType::Empty)))
    }

    #[test]
    fn parses_valid_input_with_payee_and_no_description() {
        assert_eq!(
            parse::<Error<&str>>("Test"),
            Ok(("", PayeeSectionType::PayeeOnly("Test".to_owned())))
        );

        assert_eq!(
            parse::<Error<&str>>("Test with spaces"),
            Ok((
                "",
                PayeeSectionType::PayeeOnly("Test with spaces".to_owned())
            ))
        );
    }

    #[test]
    fn parses_valid_transaction_with_payee_and_description() {
        assert_eq!(
            parse::<Error<&str>>("Test | Test description"),
            Ok((
                "",
                PayeeSectionType::PayeeAndDescription((
                    "Test".to_owned(),
                    "Test description".to_owned()
                ))
            ))
        );

        assert_eq!(
            parse::<Error<&str>>("Test with spaces | Test description"),
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
