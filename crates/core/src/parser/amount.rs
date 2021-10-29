use std::str::FromStr;

use nom::{
    bytes::complete::take_while1,
    character::complete::{alpha1, space1},
    combinator::map,
    error::{ContextError, ParseError},
    sequence::tuple,
    IResult,
};
use rust_decimal::Decimal;

use super::ast::Amount;

/// Parses an amount expressed as (-){quantity} {currency code}.
pub fn parse<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Amount, E> {
    map(
        tuple((
            take_while1(|c: char| c.is_numeric() || c == '.' || c == '-'),
            space1,
            alpha1,
        )),
        |(quantity, _, currency)| Amount {
            quantity: Decimal::from_str(quantity).unwrap(),
            currency: currency.to_owned(),
        },
    )(input)
}

#[cfg(test)]
mod test {
    use nom::{
        error::Error,
        error::ErrorKind::{Alpha, Space, TakeWhile1},
        Err,
    };
    use rust_decimal_macros::dec;

    use super::parse;

    use crate::parser::ast::Amount;

    #[test]
    fn parses_integer_amount() {
        assert_eq!(
            parse::<Error<&str>>("4 USD"),
            Ok((
                "",
                Amount {
                    quantity: dec!(4),
                    currency: "USD".to_owned()
                }
            ))
        )
    }

    #[test]
    fn parses_decimal_amount() {
        assert_eq!(
            parse::<Error<&str>>("4.05 USD"),
            Ok((
                "",
                Amount {
                    quantity: dec!(4.05),
                    currency: "USD".to_owned()
                }
            ))
        )
    }

    #[test]
    fn parses_negative_integer_amount() {
        assert_eq!(
            parse::<Error<&str>>("-4 USD"),
            Ok((
                "",
                Amount {
                    quantity: dec!(-4),
                    currency: "USD".to_owned()
                }
            ))
        )
    }

    #[test]
    fn parses_negative_decimal_amount() {
        assert_eq!(
            parse::<Error<&str>>("-4.05 USD"),
            Ok((
                "",
                Amount {
                    quantity: dec!(-4.05),
                    currency: "USD".to_owned()
                }
            ))
        )
    }

    #[test]
    fn parses_any_currency() {
        assert_eq!(
            parse::<Error<&str>>("200 A"),
            Ok((
                "",
                Amount {
                    quantity: dec!(200),
                    currency: "A".to_owned()
                }
            ))
        );

        assert_eq!(
            parse::<Error<&str>>("40 APPL"),
            Ok((
                "",
                Amount {
                    quantity: dec!(40),
                    currency: "APPL".to_owned()
                }
            ))
        );
    }

    #[test]
    fn fails_when_quantity_includes_unexpected_characters() {
        assert_eq!(
            parse::<Error<&str>>("4,04 USD"),
            Err(Err::Error(Error {
                input: ",04 USD",
                code: Space
            }))
        )
    }

    #[test]
    fn fails_when_input_starts_with_character() {
        assert_eq!(
            parse::<Error<&str>>("b4.04 USD"),
            Err(Err::Error(Error {
                input: "b4.04 USD",
                code: TakeWhile1
            }))
        )
    }

    #[test]
    fn fails_when_currency_is_missing() {
        assert_eq!(
            parse::<Error<&str>>("4.04"),
            Err(Err::Error(Error {
                input: "",
                code: Space
            }))
        );

        assert_eq!(
            parse::<Error<&str>>("4.04 "),
            Err(Err::Error(Error {
                input: "",
                code: Alpha
            }))
        )
    }
}
