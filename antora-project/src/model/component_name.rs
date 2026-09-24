use std::fmt::Display;

use super::is_valid_component_or_module_name;
use crate::Error;
use relative_path::Dirname;
use serde::{Deserialize, Serialize};

pub(crate) const HINT_COMPONENT_NAME: &str = "a component-name may only contain characters 0-9, a-z (lowercase), and '-'. It must not start or end with '-'";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ComponentName(String);

impl Display for ComponentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ComponentName {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_valid_component_or_module_name(&value) {
            Ok(ComponentName(value))
        } else {
            Err(Error::InvalidComponentName(HINT_COMPONENT_NAME.to_owned()))
        }
    }
}

impl From<ComponentName> for Dirname {
    fn from(value: ComponentName) -> Self {
        Dirname::try_from(value.0)
            .expect("creating a Dirname from a valid component-name is always valid")
    }
}

impl AsRef<str> for ComponentName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
