use crate::Error;
use regex::Regex;
use serde::{Deserialize, Serialize, de::Error as DeError};
use std::{fmt::Display, sync::LazyLock};

pub(crate) const HINT_VERSION_LITERAL: &str = "a component-version must only contain characters 0-9, a-z (lowercase), A-Z (uppercase), '.', '_' and '-'.";

fn is_valid_component_version(component_version: impl AsRef<str>) -> bool {
    static VALID_COMPONENT_VERSION: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new("[0-9a-zA-Z\\._\\-]+")
            .expect("static pattern for valid component-version must compile")
    });

    let component_version = component_version.as_ref();
    matches!(VALID_COMPONENT_VERSION.find(component_version),Some(matched) if matched.as_str() == component_version)
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ComponentVersion(Option<String>);

impl ComponentVersion {
    pub fn empty() -> Self {
        Self(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl Display for ComponentVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", AsRef::<str>::as_ref(&self))
    }
}

impl TryFrom<String> for ComponentVersion {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.as_str() == "~" {
            Ok(Self(None))
        } else if is_valid_component_version(&value) {
            Ok(Self(Some(value)))
        } else {
            Err(Error::InvalidComponentVersion(
                HINT_VERSION_LITERAL.to_owned(),
            ))
        }
    }
}

impl AsRef<str> for ComponentVersion {
    fn as_ref(&self) -> &str {
        if let Some(s) = &self.0 {
            s.as_str()
        } else {
            "~"
        }
    }
}

impl Serialize for ComponentVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for ComponentVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ComponentVersion::try_from(s).map_err(|e| DeError::custom(format!("{e}")))
    }
}
