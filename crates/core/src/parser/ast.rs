use chrono::{Date, Utc};
use rust_decimal::Decimal;

pub type CurrencyCode = String;
pub type Description = String;
pub type Payee = String;
pub type Year = u32;

#[derive(Debug, PartialEq, Clone)]
pub enum ParsedDate {
    Full(Date<Utc>),
    /// Represents dates that omitted the year and were given a default one that should be
    /// switched to the journal year.
    Partial(Date<Utc>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum JournalElement {
    Comment,
    Year(u32),
    Transaction(Transaction),
}

/// Defines the internal representation of a journal once parsed from a file.
#[derive(PartialEq, Clone)]
pub struct Journal {
    /// Optional year that applies to the whole journal.
    pub year: Option<Year>,
    pub transactions: Vec<Transaction>,
}

/// Represents an account that the journal includes. Accounts are created implicitly through their
/// usage in transactions.
#[derive(Debug, PartialEq, Clone)]
pub struct Account {
    pub name: String,
    pub children: Vec<String>,
}

/// Represents an amount with its quantity and its currency.
#[derive(Debug, PartialEq, Clone)]
pub struct Amount {
    pub quantity: Decimal,
    pub currency: CurrencyCode,
}

/// Describes the different types of sections that can appear.
#[derive(PartialEq, Debug)]
pub enum PayeeSectionType {
    Empty,
    PayeeOnly(Payee),
    PayeeAndDescription((Payee, Description)),
}

/// Defines an account posting, which indicates either a positive or negative transfer to an account.
#[derive(Debug, PartialEq, Clone)]
pub struct Posting {
    pub account: Account,
    pub amount: Option<Amount>,
}

/// Represents a transaction that happened in an user's account.
#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    pub date: ParsedDate,
    pub description: Description,
    pub payee: Payee,
    pub postings: (Posting, Posting),
}
