use core::error::Error as TError;
use core::result::Result as CoreResult;
use serde_yaml_bw::Error as YamlError;
use std::fmt::{Debug, Display};
use toml::de::Error as TomlDeError;

pub type Result<T> = CoreResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    TomlDeserializationError(TomlDeError),
    YamlDeserializationError(YamlError),
    InvalidModuleName(String),
    InvalidComponentName(String),
    InvalidComponentVersion(String),
    InvalidFamily,
    InvalidRelativePathToResource,
    NotAnOrphanRemovalStrategyVariant(String),
    NotAPublishingStrategyVariant(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TomlDeserializationError(e) => write!(f, "toml-deserialization failed: {e}"),
            Error::YamlDeserializationError(e) => write!(f, "yaml-deserialization failed: {e}"),
            Error::InvalidModuleName(hint) => write!(f, "{hint}"),
            Error::InvalidComponentName(hint) => write!(f, "{hint}"),
            Error::InvalidComponentVersion(hint) => write!(f, "{hint}"),
            Error::InvalidFamily => write!(f, "invalid family"),
            Error::InvalidRelativePathToResource => {
                write!(f, "the provided relative path to the resource is invalid")
            }
            Error::NotAnOrphanRemovalStrategyVariant(text) => {
                write!(f, "'{text}' is not a valid OrphanRemovalStrategy variant")
            }
            Error::NotAPublishingStrategyVariant(text) => {
                write!(f, "'{text}' is not a valid PublishingStrategy variant")
            }
        }
    }
}

impl TError for Error {}
