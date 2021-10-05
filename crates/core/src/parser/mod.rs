use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::Error;
use nom::sequence::delimited;
use nom::Finish;

mod ast;
mod journal_year;

/// Attempts to parse a journal from the given content, returning a result specifying
pub fn parse_journal(content: &str) -> Result<ast::Journal, Error<&str>> {
    delimited(multispace0, journal_year::parse, opt(multispace0))(content)
        .finish()
        .map(|(_, year)| ast::Journal { year: Some(year) })
}
