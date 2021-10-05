use nom::IResult;
use nom::bytes::complete::{tag_no_case};
use nom::character::complete::digit1;
use nom::error::context;
use nom::sequence::preceded;
use nom::combinator::{cut, map_res};

/// Attempts to parse a journal year. Handles an upper or lower case `Y` followed by a year and
/// returns the year.
pub fn parse(input: &str) -> IResult<&str, u32> {
    context(
        "journal year",
        preceded(
            tag_no_case("y"),
            cut(
                map_res(
                    digit1,
                    |s: &str| s.parse::<u32>(),
                )
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use nom::{Err, error::Error, error::ErrorKind::{Digit, Tag}};
    use super::parse;

    #[test]
    fn parses_lowercase_year() {
        assert_eq!(
            parse("y2021"),
            Ok(("", 2021))
        )
    }

    #[test]
    fn parses_uppercase_year() {
        assert_eq!(
            parse("Y2021"),
            Ok(("", 2021))
        )
    }
    
    #[test]
    fn passes_rest_of_input_if_year_parsed() {
        assert_eq!(
            parse("Y2022\nA"),
            Ok(("\nA", 2022))
        )
    }
    
    #[test]
    fn errors_when_empty() {
        assert_eq!(
            parse(""),
            Err(Err::Error(Error { input: "", code: Tag }))
        )
    }
    
    #[test]
    fn errors_when_wrong_tag() {
        assert_eq!(
            parse("X2020"),
            Err(Err::Error(Error { input: "X2020", code: Tag }))
        )
    }

    #[test]
    fn fails_when_invalid_year_tag() {
        assert_eq!(
            parse("YY2020"),
            Err(Err::Failure(Error { input: "Y2020", code: Digit }))
        );

        assert_eq!(
            parse("yy1920"),
            Err(Err::Failure(Error { input: "y1920", code: Digit }))
        );
    }

    #[test]
    fn fails_when_invalid_year() {
        assert_eq!(
            parse("y"),
            Err(Err::Failure(Error { input: "", code: Digit }))
        )
    }
}
