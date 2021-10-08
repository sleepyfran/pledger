use chrono::{Date, Utc};
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::{eof, map, opt};
use nom::error::Error;
use nom::multi::many_till;
use nom::sequence::{delimited, preceded};
use nom::Finish;

use self::ast::Transaction;

mod ast;
mod common;
mod journal_year;
mod transactions;

#[derive(Debug)]
enum Test {
    Year(u32),
    Transaction(Transaction),
}

/// Attempts to parse a journal from the given content, returning a result specifying
pub fn parse_journal(content: &str) -> Result<ast::Journal, Error<&str>> {
    many_till(
        preceded(
            multispace0,
            alt((
                map(journal_year::parse, Test::Year),
                map(transactions::parse, Test::Transaction),
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
