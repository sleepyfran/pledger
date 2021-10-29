use rust_decimal::Decimal;

use crate::parser::{
    ast::{JournalElement, Transaction},
    parse_journal,
};

mod checks;

/// Wraps a transaction that is unbalanced with the difference that caused it to be unbalanced.
#[derive(Debug, PartialEq)]
pub struct UnbalancedTransaction {
    pub transaction: Transaction,
    pub difference: Decimal,
}

/// Defines all the different types of checking errors that can happen when validating a journal.
#[derive(Debug, PartialEq)]
pub enum CheckError {
    NonParsable(String),
    TransactionsWithoutValue(Vec<Transaction>),
    UnbalancedTransactions(Vec<UnbalancedTransaction>),
}

/// Validates the given journal, if correct returns Ok with nothing wrapped or otherwise the
/// `CheckError` that happened during validation.
pub fn validate(content: &str) -> Result<(), CheckError> {
    parse_journal(content)
        .map_err(|error| CheckError::NonParsable(error))
        .and_then(|journal| {
            checks::transactions::check_transactions(&get_transactions(journal), "EUR".to_owned())
                .map(|_| ())
        })
}

fn get_transactions(journal: Vec<JournalElement>) -> Vec<Transaction> {
    journal
        .into_iter()
        .filter_map(|element| match element {
            JournalElement::Transaction(transaction) => Some(transaction),
            _ => None,
        })
        .collect()
}
