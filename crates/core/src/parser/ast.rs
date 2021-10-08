use chrono::{Date, Utc};
use rust_decimal::Decimal;

type Payee = String;
type Year = u32;

/// Defines the internal representation of a journal once parsed from a file.
#[derive(PartialEq)]
pub struct Journal {
    /// Optional year that applies to the whole journal.
    pub year: Option<Year>,
    pub transactions: Vec<Transaction>,
}

/// Represents an account that the journal includes. Accounts are created implicitly through their
/// usage in transactions.
#[derive(Debug, PartialEq)]
pub struct Account {
    pub name: String,
}

/// Defines an account posting, which indicates either a positive or negative transfer to an account.
#[derive(Debug, PartialEq)]
pub struct Posting {
    pub account: Account,
    pub amount: Decimal,
}

/// Represents a transaction that happened in an user's account.
#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub date: Date<Utc>,
    pub description: String,
    pub payee: Payee,
    pub postings: Vec<Posting>,
}
