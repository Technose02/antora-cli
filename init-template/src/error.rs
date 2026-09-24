use core::{error::Error as TError, result::Result as CoreResult};
use std::fmt::Display;

pub type Result<T> = CoreResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidTemplateKey(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidTemplateKey(key) => write!(f, "template-key '{key}' is invalid"),
        }
    }
}

impl Error {
    pub fn invalid_template_key(key: impl Into<String>) -> Self {
        Self::InvalidTemplateKey(key.into())
    }
}

impl TError for Error {}
