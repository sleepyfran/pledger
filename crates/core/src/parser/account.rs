use nom::{
    character::complete::{alphanumeric1, char},
    combinator::{map, opt},
    error::context,
    multi::many0,
    sequence::tuple,
    IResult,
};

use super::ast::Account;

/// Parses an account with the format "parent:child".
pub fn parse(input: &str) -> IResult<&str, Account> {
    context(
        "account",
        map(
            tuple((
                alphanumeric1,  // All accounts start with an alphanumeric name
                opt(char(':')), // Followed by an optional ':' separator
                many0(tuple((alphanumeric1, opt(char(':'))))), // And optionally repeating both
            )),
            |(account_name, _, children): (&str, Option<char>, Vec<(&str, Option<char>)>)| {
                Account {
                    name: account_name.to_owned(),
                    children: children
                        .into_iter()
                        .map(|(name, _)| name.to_owned())
                        .collect(),
                }
            },
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use nom::{error::Error, error::ErrorKind::AlphaNumeric, Err};

    use super::parse;

    use crate::parser::ast::Account;

    #[test]
    fn parses_valid_parent_only_account() {
        assert_eq!(
            parse("expenses"),
            Ok((
                "",
                Account {
                    name: "expenses".to_owned(),
                    children: Vec::new()
                }
            ))
        )
    }

    #[test]
    fn parses_valid_parent_with_colon_but_no_child_account() {
        assert_eq!(
            parse("expenses:"),
            Ok((
                "",
                Account {
                    name: "expenses".to_owned(),
                    children: Vec::new(),
                }
            ))
        )
    }

    #[test]
    fn parses_valid_parent_single_child_account() {
        assert_eq!(
            parse("expenses:food"),
            Ok((
                "",
                Account {
                    name: "expenses".to_owned(),
                    children: vec!("food".to_owned())
                }
            ))
        )
    }

    #[test]
    fn parses_valid_account_with_multiple_children() {
        assert_eq!(
            parse("assets:savings:goals:test"),
            Ok((
                "",
                Account {
                    name: "assets".to_owned(),
                    children: vec!("savings".to_owned(), "goals".to_owned(), "test".to_owned())
                }
            ))
        )
    }

    #[test]
    fn parses_valid_account_with_numbers() {
        assert_eq!(
            parse("expenses:travel:2021:10:test"),
            Ok((
                "",
                Account {
                    name: "expenses".to_owned(),
                    children: vec!(
                        "travel".to_owned(),
                        "2021".to_owned(),
                        "10".to_owned(),
                        "test".to_owned()
                    )
                }
            ))
        )
    }

    #[test]
    fn errors_when_empty() {
        assert_eq!(
            parse(""),
            Err(Err::Error(Error {
                input: "",
                code: AlphaNumeric
            }))
        )
    }

    #[test]
    fn errors_when_starts_with_colon() {
        assert_eq!(
            parse(":account"),
            Err(Err::Error(Error {
                input: ":account",
                code: AlphaNumeric
            }))
        )
    }
}
