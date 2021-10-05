use core::io::{file, file::FileError};
use core::parser;
use seahorse::{Command, Context};

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
        println!("No file given");
    }
}

fn check_file_path(path: &str) {
    let content = file::read_content(path);
    match content {
        Ok(content) => check_content(content),
        Err(error) => match error {
            FileError::NotFound => {
                println!("File not found");
            }
            FileError::Unknown => {
                println!("Unknown error while reading the file");
            }
        },
    }
}

fn check_content(content: String) {
    match parser::parse_journal(&content) {
        Ok(journal) => {
            println!("{:?}", journal)
        }
        Err(error) => {
            println!("{}", error)
        }
    }
}
