use core::{error::Error as TError, result::Result as CoreResult};
use std::fmt::Display;

pub type Result<T> = CoreResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidInitTemplateKey(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInitTemplateKey(key) => write!(f, "init_template-key '{key}' is invalid"),
        }
    }
}

impl Error {
    pub fn invalid_init_template_key(key: impl Into<String>) -> Self {
        Self::InvalidInitTemplateKey(key.into())
    }
}

impl TError for Error {}
