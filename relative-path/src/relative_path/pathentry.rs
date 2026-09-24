use serde::{Deserialize, Serialize, de::Error as DeError};

use crate::Error;
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dirname(pub(crate) String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Filename(pub(crate) String);

fn arg_is_invalid(arg: &str) -> bool {
    arg.contains("..") || arg.contains("\\") || arg.contains("/") || arg == "."
}

impl TryFrom<&str> for Dirname {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if arg_is_invalid(value) {
            Err(Error::DirnameContainsInvalidCharacters(value.to_owned()))
        } else {
            Ok(Self(value.to_owned()))
        }
    }
}

impl TryFrom<String> for Dirname {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if arg_is_invalid(&value) {
            Err(Error::DirnameContainsInvalidCharacters(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl Display for Dirname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Filename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Dirname> for String {
    fn from(value: Dirname) -> Self {
        value.0
    }
}
impl From<Dirname> for PathBuf {
    fn from(value: Dirname) -> Self {
        value.0.into()
    }
}

impl AsRef<str> for Dirname {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<Filename> for String {
    fn from(value: Filename) -> Self {
        value.0
    }
}

impl From<Filename> for PathBuf {
    fn from(value: Filename) -> PathBuf {
        value.0.into()
    }
}

impl AsRef<str> for Filename {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<&str> for Filename {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if arg_is_invalid(value) {
            Err(Error::FilenamContainsInvalidCharacters(value.to_owned()))
        } else {
            Ok(Self(value.to_owned()))
        }
    }
}

impl TryFrom<String> for Filename {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if arg_is_invalid(&value) {
            Err(Error::FilenamContainsInvalidCharacters(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl Serialize for Filename {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl Serialize for Dirname {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Dirname {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Dirname::try_from(String::deserialize(deserializer)?)
            .map_err(|e| DeError::custom(format!("{e}")))
    }
}

impl<'de> Deserialize<'de> for Filename {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Filename::try_from(String::deserialize(deserializer)?)
            .map_err(|e| DeError::custom(format!("{e}")))
    }
}
