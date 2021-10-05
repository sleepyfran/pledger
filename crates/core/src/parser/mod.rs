use nom::bytes::streaming::take_while;
use nom::combinator::opt;
use nom::error::Error;
use nom::sequence::delimited;
use nom::{Finish, IResult};

mod ast;
mod journal_year;

pub enum ParseError {
    InvalidYear(String),
}

/// Attempts to parse a journal from the given content, returning a result specifying
pub fn parse_journal(content: &str) -> Result<ast::Journal, Error<&str>> {
    delimited(spaces, journal_year::parse, opt(spaces))(content)
        .finish()
        .map(|(_, year)| ast::Journal { year: Some(year) })
}

/// Parser that consumes all spaces, new lines and return characters.
fn spaces(input: &str) -> IResult<&str, &str> {
    let space_chars = " \t\n\r";
    take_while(move |c| space_chars.contains(c))(input)
}
