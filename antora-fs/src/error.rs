use core::{error::Error as TError, result::Result as CoreResult};
use std::{
    fmt::{Debug, Display},
    io::Error as IoError,
};

pub type Result<T> = CoreResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    InitDirContainsAntoraConfiguration,
    InitDirContainsDefaultPlaybook,
    FoundSecretsConfigurationIsInvalid,
    ReadingFile(IoError),
    InvalidRelativePathPattern,
    WritingFile(IoError),
    CreatingDirs(IoError),
    NormalizeStringifyError(std::ffi::OsString),
    NormalizeCanonicalizeError(IoError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InitDirContainsAntoraConfiguration => {
                write!(f, "provided init-dir contains an antora-configuration")
            }
            Error::InitDirContainsDefaultPlaybook => {
                write!(f, "provided init-dir contains a default playbook")
            }
            Error::ReadingFile(e) => write!(f, "could not read file: {e}"),
            Error::WritingFile(e) => write!(f, "could not write file: {e}"),
            Error::CreatingDirs(e) => write!(f, "could not create directories: {e}"),
            Error::FoundSecretsConfigurationIsInvalid => write!(
                f,
                "encountered an invalid secrets-configuration in init-dir"
            ),
            Error::InvalidRelativePathPattern => write!(
                f,
                "provided pattern for RelativePath contains invalid chars",
            ),
            Error::NormalizeCanonicalizeError(e) => {
                write!(f, "failed canonicalize path: {e}")
            }
            Error::NormalizeStringifyError(e) => {
                write!(f, "failed to convert osstring: {e:#?}")
            }
        }
    }
}

impl TError for Error {}
