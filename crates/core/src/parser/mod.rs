use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::{eof, map};
use nom::error::Error;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::Finish;

mod account;
mod amount;
mod ast;
mod comment;
mod common;
mod journal_year;
mod transactions;

/// Attempts to parse a journal from the given content, returning a result specifying
pub fn parse_journal(content: &str) -> Result<ast::Journal, Error<&str>> {
    many_till(
        preceded(
            multispace0,
            alt((
                map(comment::parse, |_| ast::JournalElement::Comment),
                map(journal_year::parse, ast::JournalElement::Year),
                map(transactions::parse, ast::JournalElement::Transaction),
            )),
        ),
        eof,
    )(content)
    .finish()
    .map(|(_, result)| {
        // TODO: Remove once everything is implemented.
        println!("{:?}", result);

        return ast::Journal {
            year: Some(0),
            transactions: Vec::new(),
        };
    })
}
