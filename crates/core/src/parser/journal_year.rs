use nom::bytes::complete::tag_no_case;
use nom::character::complete::digit1;
use nom::combinator::{cut, map_res};
use nom::error::{context, ContextError, FromExternalError, ParseError};
use nom::sequence::preceded;
use nom::IResult;

/// Attempts to parse a journal year. Handles an upper or lower case `Y` followed by a year and
/// returns the year.
pub fn parse<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    input: &'a str,
) -> IResult<&'a str, u32, E> {
    context(
        "journal year",
        preceded(
            tag_no_case("y"),
            cut(map_res(digit1, |s: &str| s.parse::<u32>())),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use nom::{
        error::Error,
        error::ErrorKind::{Digit, Tag},
        Err,
    };

    #[test]
    fn parses_lowercase_year() {
        assert_eq!(parse::<Error<&str>>("y2021"), Ok(("", 2021)))
    }

    #[test]
    fn parses_uppercase_year() {
        assert_eq!(parse::<Error<&str>>("Y2021"), Ok(("", 2021)))
    }

    #[test]
    fn passes_rest_of_input_if_year_parsed() {
        assert_eq!(parse::<Error<&str>>("Y2022\nA"), Ok(("\nA", 2022)))
    }

    #[test]
    fn errors_when_empty() {
        assert_eq!(
            parse::<Error<&str>>(""),
            Err(Err::Error(Error {
                input: "",
                code: Tag
            }))
        )
    }

    #[test]
    fn errors_when_wrong_tag() {
        assert_eq!(
            parse::<Error<&str>>("X2020"),
            Err(Err::Error(Error {
                input: "X2020",
                code: Tag
            }))
        )
    }

    #[test]
    fn fails_when_invalid_year_tag() {
        assert_eq!(
            parse::<Error<&str>>("YY2020"),
            Err(Err::Failure(Error {
                input: "Y2020",
                code: Digit
            }))
        );

        assert_eq!(
            parse::<Error<&str>>("yy1920"),
            Err(Err::Failure(Error {
                input: "y1920",
                code: Digit
            }))
        );
    }

    #[test]
    fn fails_when_invalid_year() {
        assert_eq!(
            parse("y"),
            Err(Err::Failure(Error {
                input: "",
                code: Digit
            }))
        )
    }
}
