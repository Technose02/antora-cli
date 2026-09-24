use super::is_valid_component_or_module_name;
use crate::Error;
use relative_path::Dirname;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::LazyLock};

const HINT_MODULE_NAME: &str = "a module-name may only contain characters 0-9, a-z (lowercase), and '-'. It must not start or end with '-'";

pub static ROOT_MODULE: LazyLock<ModuleName> = LazyLock::new(|| ModuleName("ROOT".to_owned()));

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleName(String);

impl Display for ModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ModuleName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<&str> for ModuleName {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == ROOT_MODULE.0 {
            Ok(ROOT_MODULE.clone())
        } else if is_valid_component_or_module_name(value) {
            Ok(ModuleName(value.to_owned()))
        } else {
            Err(Error::InvalidModuleName(HINT_MODULE_NAME.to_owned()))
        }
    }
}

impl TryFrom<String> for ModuleName {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == ROOT_MODULE.0 {
            Ok(ROOT_MODULE.clone())
        } else if is_valid_component_or_module_name(&value) {
            Ok(ModuleName(value))
        } else {
            Err(Error::InvalidModuleName(HINT_MODULE_NAME.to_owned()))
        }
    }
}

impl From<ModuleName> for Dirname {
    fn from(value: ModuleName) -> Self {
        Dirname::try_from(value.0)
            .expect("creating a Dirname from a valid module-name is always valid")
    }
}
