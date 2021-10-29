use crate::conversion::convert_to;
use crate::parser::ast::{CurrencyCode, Transaction};

use crate::journal::{CheckError, UnbalancedTransaction};

/// Checks that all the given transactions are contain at least one value and that they are balanced,
/// which means that the sum of all the specified quantities equals 0.
///
/// If any transaction is found to be unbalanced or without at least one value defined, the function
/// returns a CheckError::UnbalancedTransaction with the list of transactions that were found,
/// otherwise it returns the given list of transactions.
pub fn check_transactions<'t>(
    transactions: &'t [Transaction],
    base_currency: CurrencyCode,
) -> Result<&'t [Transaction], CheckError> {
    let invalid_transactions = transactions_without_value(transactions);

    if !invalid_transactions.is_empty() {
        return Err(CheckError::TransactionsWithoutValue(
            invalid_transactions.into_iter().cloned().collect(),
        ));
    }

    let unbalanced_transactions = unbalanced_transactions(transactions, &base_currency);

    if unbalanced_transactions.is_empty() {
        Ok(transactions)
    } else {
        Err(CheckError::UnbalancedTransactions(unbalanced_transactions))
    }
}

fn transactions_without_value(transactions: &[Transaction]) -> Vec<&Transaction> {
    transactions
        .into_iter()
        .filter(|&transaction| {
            transaction.postings.0.amount.is_none() && transaction.postings.1.amount.is_none()
        })
        .collect()
}

fn unbalanced_transactions(
    transactions: &[Transaction],
    base_currency: &CurrencyCode,
) -> Vec<UnbalancedTransaction> {
    transactions
        .into_iter()
        .filter_map(|transaction| unbalanced_transaction(transaction, base_currency))
        .collect()
}

fn unbalanced_transaction(
    transaction: &Transaction,
    base_currency: &CurrencyCode,
) -> Option<UnbalancedTransaction> {
    // Transactions are guaranteed to have at least one value after passing by the previous validation
    // so attempt to fetch at least one amount if the posting does not have a value.
    let first_amount = transaction.postings.0.amount.as_ref().or(transaction
        .postings
        .1
        .amount
        .as_ref());
    let second_amount = transaction.postings.1.amount.as_ref().or(transaction
        .postings
        .0
        .amount
        .as_ref());

    // Convert to the base currency to be able to compare them.
    let first_amount = convert_to(base_currency, &first_amount.unwrap());
    let second_amount = convert_to(base_currency, &second_amount.unwrap());

    if first_amount.quantity == second_amount.quantity {
        None
    } else {
        let difference = first_amount.quantity - second_amount.quantity;
        Some(UnbalancedTransaction {
            transaction: transaction.clone(),
            difference: difference.abs(),
        })
    }
}

#[cfg(test)]
mod test {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use crate::journal::*;
    use crate::parser::ast::*;

    use super::check_transactions;

    fn balanced_transaction(quantity: Decimal) -> Transaction {
        Transaction {
            postings: (
                Posting {
                    account: Account::default(),
                    amount: Some(Amount {
                        quantity,
                        currency: "EUR".to_owned(),
                    }),
                },
                Posting::default(),
            ),
            ..Transaction::default()
        }
    }

    fn unbalanced_transaction(quantity: Decimal) -> Transaction {
        Transaction {
            postings: (
                Posting {
                    account: Account::default(),
                    amount: Some(Amount {
                        quantity,
                        currency: "EUR".to_owned(),
                    }),
                },
                Posting {
                    account: Account::default(),
                    amount: Some(Amount {
                        quantity: quantity + dec!(10.0),
                        currency: "EUR".to_owned(),
                    }),
                },
            ),
            ..Transaction::default()
        }
    }

    fn different_currencies_transaction(
        quantity: Decimal,
        first_currency: CurrencyCode,
        second_currency: CurrencyCode,
    ) -> Transaction {
        Transaction {
            postings: (
                Posting {
                    account: Account::default(),
                    amount: Some(Amount {
                        quantity,
                        currency: first_currency,
                    }),
                },
                Posting {
                    account: Account::default(),
                    amount: Some(Amount {
                        quantity: quantity,
                        currency: second_currency,
                    }),
                },
            ),
            ..Transaction::default()
        }
    }

    fn transaction_without_value() -> Transaction {
        Transaction {
            postings: (
                Posting {
                    account: Account::default(),
                    amount: None,
                },
                Posting::default(),
            ),
            ..Transaction::default()
        }
    }

    #[test]
    fn check_transactions_should_pass_if_transactions_are_valid_and_balanced() {
        let valid_transactions = vec![
            balanced_transaction(dec!(10.0)),
            balanced_transaction(dec!(20.0)),
        ];

        assert_eq!(
            check_transactions(&valid_transactions, "CZK".to_owned()),
            Ok(valid_transactions.as_slice())
        )
    }

    #[test]
    fn check_transactions_should_pass_with_valid_and_balanced_transactions_in_different_currencies()
    {
        let valid_transactions = vec![
            different_currencies_transaction(dec!(10.0), "EUR".to_owned(), "CZK".to_owned()),
            different_currencies_transaction(dec!(10.0), "CZK".to_owned(), "EUR".to_owned()),
        ];

        assert_eq!(
            check_transactions(&valid_transactions, "CZK".to_owned()),
            Ok(valid_transactions.as_slice())
        )
    }

    #[test]
    fn check_transactions_should_fail_with_transactions_without_value_if_none_of_the_postings_contains_an_amount(
    ) {
        let invalid_transactions = vec![
            balanced_transaction(dec!(10.0)),
            transaction_without_value(),
            balanced_transaction(dec!(20.0)),
            transaction_without_value(),
        ];

        assert_eq!(
            check_transactions(&invalid_transactions, "CZK".to_owned()),
            Err(CheckError::TransactionsWithoutValue(vec![
                transaction_without_value(),
                transaction_without_value()
            ]))
        )
    }

    #[test]
    fn check_transactions_should_fail_with_unbalanced_transactions() {
        let unbalanced_transactions = vec![
            balanced_transaction(dec!(10.0)),
            unbalanced_transaction(dec!(10.0)),
            balanced_transaction(dec!(20.0)),
            unbalanced_transaction(dec!(100.0)),
        ];

        assert_eq!(
            check_transactions(&unbalanced_transactions, "CZK".to_owned()),
            Err(CheckError::UnbalancedTransactions(vec![
                UnbalancedTransaction {
                    transaction: unbalanced_transaction(dec!(10.0)),
                    difference: dec!(10.0)
                },
                UnbalancedTransaction {
                    transaction: unbalanced_transaction(dec!(100.0)),
                    difference: dec!(10.0)
                },
            ]))
        )
    }
}
