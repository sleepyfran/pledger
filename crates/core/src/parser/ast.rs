use chrono::{Date, Datelike, Utc};
use rust_decimal::Decimal;
use std::fmt::{self, Display};

pub type CurrencyCode = String;
pub type Description = String;
pub type Payee = String;
pub type Tag = String;
pub type Year = u32;

#[derive(Debug, PartialEq, Clone)]
pub enum ParsedDate {
    Full(Date<Utc>),
    /// Represents dates that omitted the year and were given a default one that should be
    /// switched to the journal year.
    Partial(Date<Utc>),
}

impl Display for ParsedDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ParsedDate::Full(date) => write!(f, "{}/{}", date.day(), date.month()),
            &ParsedDate::Partial(date) => write!(f, "{}/{}", date.day(), date.month()),
        }
    }
}

impl Default for ParsedDate {
    fn default() -> Self {
        Self::Full(Utc::now().date())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum JournalElement {
    Empty,
    Account(Account),
    Comment,
    Year(Year),
    Transaction(Transaction),
}

impl fmt::Display for JournalElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JournalElement::Empty => write!(f, ""),
            JournalElement::Account(account) => write!(f, "Account: {:?}", account),
            JournalElement::Comment => write!(f, "Comment\n"),
            JournalElement::Year(year) => write!(f, "Year: {}\n", year),
            JournalElement::Transaction(transaction) => write!(f, "{:?}\n", transaction),
        }
    }
}

/// Represents an account that the journal includes. Accounts are created implicitly through their
/// usage in transactions.
#[derive(Debug, PartialEq, Clone, Default)]
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
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Posting {
    pub account: Account,
    pub amount: Option<Amount>,
}

/// Defines the different statuses a transaction can have.
#[derive(PartialEq, Debug, Clone)]
pub enum TransactionStatus {
    Cleared,
    Pending,
}

impl Default for TransactionStatus {
    fn default() -> Self {
        Self::Cleared
    }
}

/// Represents a transaction that happened in an user's account.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Transaction {
    pub date: ParsedDate,
    pub status: TransactionStatus,
    pub description: Description,
    pub tags: Vec<Tag>,
    pub payee: Payee,
    pub postings: (Posting, Posting),
}
