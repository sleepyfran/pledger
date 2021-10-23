use nom::{
    character::complete::{line_ending, space0},
    combinator::{map, opt},
    error::{context, ContextError, ParseError},
    sequence::{terminated, tuple},
    IResult,
};

use crate::parser::{account, amount, ast::Posting};

/// Attempts to parse multiple postings divided by a line ending.
pub fn parse_multiple<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (Posting, Posting), E> {
    context(
        "postings",
        tuple((
            terminated(parse_one, line_ending),
            terminated(parse_one, opt(line_ending)),
        )),
    )(input)
}

/// Attempts to parse a posting, ignoring any white space that comes before and stopping once a line
/// ending is found.
pub fn parse_one<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Posting, E> {
    context(
        "posting",
        map(
            tuple((terminated(account::parse, space0), opt(amount::parse))),
            |(account, amount)| Posting { account, amount },
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use nom::{
        error::Error,
        error::ErrorKind::{AlphaNumeric, CrLf},
        Err,
    };
    use rust_decimal_macros::dec;

    use super::{parse_multiple, parse_one};

    use crate::parser::ast::{Account, Amount, Posting};

    fn get_test_data() -> (Account, Account, Amount) {
        let sender_account = Account {
            name: "test".to_owned(),
            children: vec!["sender".to_owned()],
        };
        let receiver_account = Account {
            name: "test".to_owned(),
            children: vec!["receiver".to_owned()],
        };
        let amount = Amount {
            quantity: dec!(4.05),
            currency: "USD".to_owned(),
        };

        (sender_account, receiver_account, amount)
    }

    #[test]
    fn parses_valid_posting() {
        let (_, receiver_account, amount) = get_test_data();
        assert_eq!(
            parse_one::<Error<&str>>("test:receiver 4.05 USD"),
            Ok((
                "",
                Posting {
                    account: receiver_account,
                    amount: Some(amount)
                }
            ))
        )
    }

    #[test]
    fn parses_valid_postings() {
        let (sender_account, receiver_account, amount) = get_test_data();
        assert_eq!(
            parse_multiple::<Error<&str>>("test:receiver 4.05 USD\ntest:sender -4.05 USD\n"),
            Ok((
                "",
                (
                    Posting {
                        account: receiver_account,
                        amount: Some(amount)
                    },
                    Posting {
                        account: sender_account,
                        amount: Some(Amount {
                            quantity: dec!(-4.05),
                            currency: "USD".to_owned()
                        })
                    }
                )
            ))
        )
    }

    #[test]
    fn parses_valid_postings_without_amount_in_second_posting() {
        let (sender_account, receiver_account, amount) = get_test_data();
        assert_eq!(
            parse_multiple::<Error<&str>>("test:receiver 4.05 USD\ntest:sender\n"),
            Ok((
                "",
                (
                    Posting {
                        account: receiver_account,
                        amount: Some(amount)
                    },
                    Posting {
                        account: sender_account,
                        amount: None
                    }
                )
            ))
        )
    }

    #[test]
    fn parses_valid_postings_without_line_ending_in_second_posting() {
        let (sender_account, receiver_account, amount) = get_test_data();
        assert_eq!(
            parse_multiple::<Error<&str>>("test:receiver 4.05 USD\ntest:sender"),
            Ok((
                "",
                (
                    Posting {
                        account: receiver_account,
                        amount: Some(amount)
                    },
                    Posting {
                        account: sender_account,
                        amount: None
                    }
                )
            ))
        )
    }

    #[test]
    fn errors_if_posting_begins_with_space() {
        assert_eq!(
            parse_one::<Error<&str>>(" test:receiver 4.05 USD"),
            Err(Err::Error(Error {
                input: " test:receiver 4.05 USD",
                code: AlphaNumeric
            }))
        )
    }

    #[test]
    fn errors_if_posting_is_not_separated_by_new_lines() {
        assert_eq!(
            parse_multiple("test:receiver 4.05 USD test:sender"),
            Err(Err::Error(Error {
                input: " test:sender",
                code: CrLf
            }))
        )
    }

    #[test]
    fn errors_if_posting_is_separated_by_more_than_one_new_line() {
        assert_eq!(
            parse_multiple("test:receiver 4.05 USD\n\ntest:sender"),
            Err(Err::Error(Error {
                input: "\ntest:sender",
                code: AlphaNumeric
            }))
        )
    }

    #[test]
    fn errors_if_postings_have_spaces_in_between() {
        assert_eq!(
            parse_multiple("test:receiver 4.05 USD\n test:sender"),
            Err(Err::Error(Error {
                input: " test:sender",
                code: AlphaNumeric
            }))
        );

        assert_eq!(
            parse_multiple("test:receiver 4.05 USD \ntest:sender"),
            Err(Err::Error(Error {
                input: " \ntest:sender",
                code: CrLf
            }))
        )
    }
}
