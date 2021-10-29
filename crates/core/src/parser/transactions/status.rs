use nom::{
    character::complete::char,
    combinator::{map, opt},
    error::{context, ContextError, ParseError},
    IResult,
};

use crate::parser::ast::TransactionStatus;

/// Parses an optional bang that represents that the transaction is pending. If the bang is not in
/// place or the input includes any other input, it's ignored and assumed to be cleared.
pub fn parse<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, TransactionStatus, E> {
    context(
        "status",
        map(opt(char('!')), |status| match status {
            Some(_) => TransactionStatus::Pending,
            None => TransactionStatus::Cleared,
        }),
    )(input)
}

#[cfg(test)]
mod test {
    use nom::error::Error;

    use super::parse;
    use crate::parser::ast::TransactionStatus;

    #[test]
    fn parses_empty_status_as_cleared() {
        assert_eq!(
            parse::<Error<&str>>(""),
            Ok(("", TransactionStatus::Cleared))
        )
    }

    #[test]
    fn ignores_any_other_input_as_cleared() {
        assert_eq!(
            parse::<Error<&str>>("Test of ignored input"),
            Ok(("Test of ignored input", TransactionStatus::Cleared))
        )
    }

    #[test]
    fn parses_bang_as_pending() {
        assert_eq!(
            parse::<Error<&str>>("!"),
            Ok(("", TransactionStatus::Pending))
        )
    }
}
