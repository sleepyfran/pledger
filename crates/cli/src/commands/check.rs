use core::io::{file, file::FileError};
use core::parser;
use seahorse::{Command, Context};

use crate::emoji;
use crate::io::{self, show_error, show_info, show_success};

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
    match parser::parse_journal(&content) {
        Ok(elements) => {
            show_success(emoji::for_success(), "The given journal is a valid file");
            show_info(
                emoji::for_success(),
                elements
                    .iter()
                    .map(|element| element.to_string())
                    .collect::<Vec<String>>()
                    .join("> "),
            )
        }
        Err(error) => {
            show_error(emoji::for_error(), format!("{}", error));
            std::process::exit(1);
        }
    }
}
