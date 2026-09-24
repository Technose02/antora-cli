use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    FilenamContainsInvalidCharacters(String),
    DirnameContainsInvalidCharacters(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FilenamContainsInvalidCharacters(filename) => write!(
                f,
                "provided filename '{filename}' contains invalid characters"
            ),
            Error::DirnameContainsInvalidCharacters(dirname) => write!(
                f,
                "provided dirname '{dirname}' contains invalid characters"
            ),
        }
    }
}

impl core::error::Error for Error {}
