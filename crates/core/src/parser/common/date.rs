use chrono::{Date, NaiveDate, Utc};
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::multispace0,
    combinator::{map, map_res},
    error::context,
    sequence::terminated,
    IResult,
};

/// Attempts to parse a date from the given input.
pub fn parse(input: &str) -> IResult<&str, Date<Utc>> {
    context(
        "date",
        terminated(
            alt((
                map(parse_hyphen_separated_date, map_to_utc),
                map(parse_period_separated_date, map_to_utc),
                map(parse_slash_separated_date, map_to_utc),
            )),
            multispace0,
        ),
    )(input)
}

fn parse_hyphen_separated_date(input: &str) -> IResult<&str, NaiveDate> {
    context(
        "hyphen separated date",
        map_res(take_while1(|c: char| c.is_numeric() || c == '-'), |s| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
        }),
    )(input)
}

fn parse_period_separated_date(input: &str) -> IResult<&str, NaiveDate> {
    context(
        "period separated date",
        map_res(take_while1(|c: char| c.is_numeric() || c == '.'), |s| {
            NaiveDate::parse_from_str(s, "%Y.%m.%d")
        }),
    )(input)
}

fn parse_slash_separated_date(input: &str) -> IResult<&str, NaiveDate> {
    context(
        "slash separated date",
        map_res(take_while1(|c: char| c.is_numeric() || c == '/'), |s| {
            NaiveDate::parse_from_str(s, "%Y/%m/%d")
        }),
    )(input)
}

fn map_to_utc(input: NaiveDate) -> Date<Utc> {
    Date::<Utc>::from_utc(input, Utc)
}

#[cfg(test)]
mod tests {
    use super::parse;

    use chrono::{Date, NaiveDate, Utc};
    use nom::{
        error::Error,
        error::ErrorKind::{MapRes, TakeWhile1},
        Err,
    };

    #[test]
    fn parses_hyphen_separated_date() {
        let date = NaiveDate::from_ymd(2021, 10, 7);
        assert_eq!(
            parse("2021-10-07"),
            Ok(("", Date::<Utc>::from_utc(date, Utc)))
        )
    }

    #[test]
    fn parses_period_separated_date() {
        let date = NaiveDate::from_ymd(2021, 10, 7);
        assert_eq!(
            parse("2021.10.07"),
            Ok(("", Date::<Utc>::from_utc(date, Utc)))
        )
    }

    #[test]
    fn parses_slash_separated_date() {
        let date = NaiveDate::from_ymd(2021, 10, 7);
        assert_eq!(
            parse("2021/10/07"),
            Ok(("", Date::<Utc>::from_utc(date, Utc)))
        )
    }

    #[test]
    fn parses_date_and_returns_rest_of_line() {
        let date = NaiveDate::from_ymd(2021, 10, 7);
        assert_eq!(
            parse("2021/10/07 test"),
            Ok(("test", Date::<Utc>::from_utc(date, Utc)))
        )
    }

    #[test]
    fn errors_when_empty() {
        assert_eq!(
            parse(""),
            Err(Err::Error(Error {
                input: "",
                code: TakeWhile1
            }))
        )
    }

    #[test]
    fn errors_when_date_starts_with_space() {
        assert_eq!(
            parse(" 2021-10-07"),
            Err(Err::Error(Error {
                input: " 2021-10-07",
                code: TakeWhile1
            }))
        )
    }

    #[test]
    fn errors_when_date_is_invalid() {
        assert_eq!(
            parse("2020#02#01"),
            Err(Err::Error(Error {
                input: "2020#02#01",
                code: MapRes
            }))
        )
    }
}
