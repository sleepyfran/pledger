use chrono::{Date, NaiveDate, Utc};
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::multispace0,
    combinator::{map, map_res},
    error::{context, ContextError, FromExternalError, ParseError},
    sequence::terminated,
    IResult,
};

use crate::parser::ast::ParsedDate;

/// Attempts to parse a date from the given input.
pub fn parse<
    'a,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, chrono::ParseError>,
>(
    input: &'a str,
) -> IResult<&'a str, ParsedDate, E> {
    context(
        "date",
        terminated(
            alt((
                map(|i| parse_full_date('-', i), map_to_utc),
                map(|i| parse_full_date('/', i), map_to_utc),
                map(|i| parse_full_date('.', i), map_to_utc),
                map(|i| parse_partial_date('-', i), map_to_partial_date),
                map(|i| parse_partial_date('/', i), map_to_partial_date),
                map(|i| parse_partial_date('.', i), map_to_partial_date),
            )),
            multispace0,
        ),
    )(input)
}

fn parse_full_date<
    'a,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, chrono::ParseError>,
>(
    separator: char,
    input: &'a str,
) -> IResult<&'a str, NaiveDate, E> {
    context(
        "full date",
        map_res(
            take_while1(|c: char| c.is_numeric() || c == separator),
            |s| NaiveDate::parse_from_str(s, &format!("%Y{0}%m{0}%d", separator)),
        ),
    )(input)
}

fn parse_partial_date<
    'a,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, chrono::ParseError>,
>(
    separator: char,
    input: &'a str,
) -> IResult<&'a str, NaiveDate, E> {
    context(
        "partial date",
        map_res(
            take_while1(|c: char| c.is_numeric() || c == separator),
            |s| {
                NaiveDate::parse_from_str(
                    &format!("2021{}{}", separator, s), // Set a false 2021 that will be replaced later.
                    &format!("%Y{0}%m{0}%d", separator),
                )
            },
        ),
    )(input)
}

fn map_to_utc(input: NaiveDate) -> ParsedDate {
    ParsedDate::Full(Date::<Utc>::from_utc(input, Utc))
}

fn map_to_partial_date(input: NaiveDate) -> ParsedDate {
    ParsedDate::Partial(Date::<Utc>::from_utc(input, Utc))
}

#[cfg(test)]
mod tests {
    use super::{parse, ParsedDate};

    use chrono::{Date, NaiveDate, Utc};
    use nom::{
        error::Error,
        error::ErrorKind::{MapRes, TakeWhile1},
        Err,
    };

    fn get_full_test_date<'a>(separator: char) -> (String, Date<Utc>) {
        let date = NaiveDate::from_ymd(2021, 10, 7);
        (
            format!("2021{0}10{0}07", separator),
            Date::<Utc>::from_utc(date, Utc),
        )
    }

    fn get_partial_test_date<'a>(separator: char) -> (String, Date<Utc>) {
        let date = NaiveDate::from_ymd(2021, 10, 7);
        (
            format!("10{0}07", separator),
            Date::<Utc>::from_utc(date, Utc),
        )
    }

    #[test]
    fn parses_full_hyphen_separated_date() {
        let (input, expected_date) = get_full_test_date('-');
        assert_eq!(
            parse::<Error<&str>>(&input),
            Ok(("", ParsedDate::Full(expected_date)))
        )
    }

    #[test]
    fn parses_full_period_separated_date() {
        let (input, expected_date) = get_full_test_date('.');
        assert_eq!(
            parse::<Error<&str>>(&input),
            Ok(("", ParsedDate::Full(expected_date)))
        )
    }

    #[test]
    fn parses_full_slash_separated_date() {
        let (input, expected_date) = get_full_test_date('/');
        assert_eq!(
            parse::<Error<&str>>(&input),
            Ok(("", ParsedDate::Full(expected_date)))
        )
    }

    #[test]
    fn parses_partial_hyphen_separated_date() {
        let (input, expected_date) = get_partial_test_date('-');
        assert_eq!(
            parse::<Error<&str>>(&input),
            Ok(("", ParsedDate::Partial(expected_date)))
        )
    }

    #[test]
    fn parses_partial_period_separated_date() {
        let (input, expected_date) = get_partial_test_date('.');
        assert_eq!(
            parse::<Error<&str>>(&input),
            Ok(("", ParsedDate::Partial(expected_date)))
        )
    }

    #[test]
    fn parses_partial_slash_separated_date() {
        let (input, expected_date) = get_partial_test_date('/');
        assert_eq!(
            parse::<Error<&str>>(&input),
            Ok(("", ParsedDate::Partial(expected_date)))
        )
    }

    #[test]
    fn parses_date_and_returns_rest_of_line() {
        let (input, expected_date) = get_full_test_date('-');
        assert_eq!(
            parse::<Error<&str>>(&format!("{} test", input)),
            Ok(("test", ParsedDate::Full(expected_date)))
        )
    }

    #[test]
    fn errors_when_empty() {
        assert_eq!(
            parse::<Error<&str>>(""),
            Err(Err::Error(Error {
                input: "",
                code: TakeWhile1
            }))
        )
    }

    #[test]
    fn errors_when_date_starts_with_space() {
        assert_eq!(
            parse::<Error<&str>>(" 2021-10-07"),
            Err(Err::Error(Error {
                input: " 2021-10-07",
                code: TakeWhile1
            }))
        )
    }

    #[test]
    fn errors_when_date_is_invalid() {
        assert_eq!(
            parse::<Error<&str>>("2020#02#01"),
            Err(Err::Error(Error {
                input: "2020#02#01",
                code: MapRes
            }))
        )
    }
}
