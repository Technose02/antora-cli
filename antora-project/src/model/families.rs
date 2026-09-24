use crate::Error;
use relative_path::Dirname;
use serde::{Deserialize, Serialize, de::Error as DeError};
use std::fmt::Display;

pub const ATTACHMENTS: &str = "attachments";
pub const EXAMPLES: &str = "examples";
pub const IMAGES: &str = "images";
pub const PAGES: &str = "pages";
pub const PARTIALS: &str = "partials";

#[derive(Debug, Clone, PartialEq)]
pub enum Families {
    Attachments,
    Examples,
    Images,
    Pages,
    Partials,
}

impl Display for Families {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Families::Attachments => write!(f, "{ATTACHMENTS}"),
            Families::Examples => write!(f, "{EXAMPLES}"),
            Families::Images => write!(f, "{IMAGES}"),
            Families::Pages => write!(f, "{PAGES}"),
            Families::Partials => write!(f, "{PARTIALS}"),
        }
    }
}

impl From<Families> for Dirname {
    fn from(value: Families) -> Self {
        Dirname::try_from(value.to_string())
            .expect("creating a dirname from a family is always valid")
    }
}

impl TryFrom<&str> for Families {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ATTACHMENTS => Ok(Families::Attachments),
            EXAMPLES => Ok(Families::Examples),
            IMAGES => Ok(Families::Images),
            PAGES => Ok(Families::Pages),
            PARTIALS => Ok(Families::Partials),
            _ => Err(Error::InvalidFamily),
        }
    }
}

impl Serialize for Families {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Families {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Families::try_from(value.as_str())
            .map_err(|e| DeError::custom(format!("failed to deserialize family: {e}")))
    }
}
