use core::io::{file, file::FileError};
use core::journal::{self, CheckError};
use seahorse::{Command, Context};

use crate::emoji;
use crate::io::{self, show_error, show_success};

/// Creates a command that attempts to parse a given journal file and shows the result of the parsing.
pub fn create() -> Command {
    Command::new("check")
        .alias("c")
        .usage("[file path] Checks that the given journal file is valid")
        .action(handler)
}

fn handler(context: &Context) {
    if let Some(file_path) = context.args.first() {
        check_file_path(file_path);
    } else {
        io::show_error(emoji::for_error(), "No file given");
    }
}

fn check_file_path(path: &str) {
    let content = file::read_content(path);
    match content {
        Ok(content) => check_content(content),
        Err(error) => match error {
            FileError::NotFound => {
                show_error(emoji::for_search(), format!("File \"{}\" not found", path))
            }
            FileError::Unknown => {
                show_error(emoji::for_error(), "Unknown error while reading the file");
            }
        },
    }
}

fn check_content(content: String) {
    match journal::validate(&content) {
        Ok(_) => {
            show_success(emoji::for_success(), "The given journal is a valid file");
        }
        Err(error) => {
            show_validate_error(error);
            std::process::exit(1);
        }
    }
}

fn show_validate_error(error: CheckError) {
    match error {
        CheckError::NonParsable(err) => show_error(
            emoji::for_error(),
            format!("There was an error parsing the journal:\n{}", err),
        ),
        CheckError::TransactionsWithoutValue(transactions) => {
            show_error(
                emoji::for_error(),
                "The following transactions have no values associated with them".to_owned(),
            );
            show_error(
                emoji::for_error(),
                transactions
                    .into_iter()
                    .map(|transaction| {
                        format!(
                            "- Payee: {}, date: {:?}",
                            transaction.payee, transaction.date
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
        CheckError::UnbalancedTransactions(transactions) => {
            show_error(
                emoji::for_error(),
                "The following transactions are unbalanced".to_owned(),
            );
            show_error(
                emoji::for_error(),
                transactions
                    .into_iter()
                    .map(|unbalanced_transaction| {
                        format!(
                            "- Payee: {}, date: {}. Difference between postings: {}",
                            unbalanced_transaction.transaction.payee,
                            unbalanced_transaction.transaction.date,
                            unbalanced_transaction.difference
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
    }
}
