use crate::{Error, Result};
use std::path::Path;

pub trait NormalizePath {
    fn try_normalize(&self) -> Result<String>;
}

impl<T> NormalizePath for T
where
    T: AsRef<Path>,
{
    fn try_normalize(&self) -> Result<String> {
        let canonicalized = self
            .as_ref()
            .canonicalize()
            .map_err(Error::NormalizeCanonicalizeError)?;

        // Convert to String (handle possible invalid UTF-8)
        let mut path_str = canonicalized
            .into_os_string()
            .into_string()
            .map_err(Error::NormalizeStringifyError)?;

        // Strip the Windows extended-length prefix if present
        const PREFIX: &str = r"\\?\";
        if path_str.starts_with(PREFIX) {
            path_str = path_str[PREFIX.len()..].to_owned();
        }

        // Replace backslashes with forward slashes
        let normalized = path_str.replace('\\', "/");

        Ok(normalized)
    }
}
