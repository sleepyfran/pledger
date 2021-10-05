use std::{io, fs};

/// Different errors that can happen while reading a file.
pub enum FileError {
    NotFound,
    Unknown
}

/// Attempts to open and read the content of the given file path. Returns a `FileError` if there
/// was something wrong while reading the file.
pub fn read_content(path: &str) -> Result<String, FileError> {
    fs::read_to_string(path)
        .map_err(|err: io::Error| {
            match err.kind() {
                io::ErrorKind::NotFound => FileError::NotFound,
                _ => FileError::Unknown
            }
        })
}