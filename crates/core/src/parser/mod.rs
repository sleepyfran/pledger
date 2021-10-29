use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space1};
use nom::combinator::{eof, map};
use nom::error::convert_error;
use nom::multi::many_till;
use nom::sequence::{preceded, tuple};
use nom::Finish;

mod account;
mod amount;
pub mod ast;
mod comment;
mod common;
mod journal_year;
mod transactions;

/// Attempts to parse a journal from the given content, returning a result specifying
pub fn parse_journal(content: &str) -> Result<Vec<ast::JournalElement>, String> {
    many_till(
        preceded(
            multispace0,
            alt((
                map(
                    preceded(tuple((tag("account"), space1)), account::parse),
                    ast::JournalElement::Account,
                ),
                map(comment::parse, |_| ast::JournalElement::Comment),
                map(journal_year::parse, ast::JournalElement::Year),
                map(transactions::parse, ast::JournalElement::Transaction),
                map(multispace0, |_| ast::JournalElement::Empty),
            )),
        ),
        eof,
    )(content)
    .finish()
    .map(|(_, (elements, _))| elements)
    .map_err(|err| convert_error(content, err))
}

#[cfg(test)]
mod test {
    use super::parse_journal;

    use crate::parser::ast;

    #[test]
    fn parses_valid_account_declaration() {
        assert_eq!(
            parse_journal("account test:test2"),
            Ok(vec![ast::JournalElement::Account(ast::Account {
                name: "test".to_string(),
                children: vec!["test2".to_owned()]
            })])
        )
    }

    #[test]
    fn fails_if_account_does_not_contain_space() {
        assert_eq!(
            parse_journal("accounttest"),
            Err("0: at line 1, in ManyTill:\naccounttest\n^\n\n".to_owned())
        )
    }

    #[test]
    fn fails_if_does_not_contain_account_name() {
        assert_eq!(
            parse_journal("account"),
            Err("0: at line 1, in ManyTill:\naccount\n^\n\n".to_owned())
        )
    }
}
