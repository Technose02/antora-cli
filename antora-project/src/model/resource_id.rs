use crate::{
    Error, ROOT_MODULE_NAME,
    component_name::ComponentName,
    component_version::ComponentVersion,
    families::{ATTACHMENTS, EXAMPLES, Families, IMAGES, PAGES, PARTIALS},
    module_name::{ModuleName, ROOT_MODULE},
};
use relative_path::{
    Dirname, Filename, RelativeFile, create_relative_dir_from_dirname,
    create_relative_file_from_filename,
};
use serde::{Deserialize, Serialize, de::Error as DeError};
use std::fmt::Display;

use super::component_name::HINT_COMPONENT_NAME;

#[derive(Debug, PartialEq, Default)]
pub enum ResourceIdDetailLevel {
    RelativeFile,
    Module,
    #[default]
    ComponentName,
    ComponentVersion,
}

impl PartialOrd for ResourceIdDetailLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ResourceIdDetailLevel::RelativeFile, ResourceIdDetailLevel::RelativeFile) => {
                Some(std::cmp::Ordering::Equal)
            }
            (ResourceIdDetailLevel::RelativeFile, _) => Some(std::cmp::Ordering::Less),

            (ResourceIdDetailLevel::Module, ResourceIdDetailLevel::RelativeFile) => {
                Some(std::cmp::Ordering::Greater)
            }
            (ResourceIdDetailLevel::Module, ResourceIdDetailLevel::Module) => {
                Some(std::cmp::Ordering::Equal)
            }
            (ResourceIdDetailLevel::Module, _) => Some(std::cmp::Ordering::Less),

            (ResourceIdDetailLevel::ComponentName, ResourceIdDetailLevel::RelativeFile) => {
                Some(std::cmp::Ordering::Greater)
            }
            (ResourceIdDetailLevel::ComponentName, ResourceIdDetailLevel::Module) => {
                Some(std::cmp::Ordering::Greater)
            }
            (ResourceIdDetailLevel::ComponentName, ResourceIdDetailLevel::ComponentName) => {
                Some(std::cmp::Ordering::Equal)
            }
            (ResourceIdDetailLevel::ComponentName, _) => Some(std::cmp::Ordering::Less),

            (ResourceIdDetailLevel::ComponentVersion, ResourceIdDetailLevel::ComponentVersion) => {
                Some(std::cmp::Ordering::Equal)
            }
            (ResourceIdDetailLevel::ComponentVersion, _) => Some(std::cmp::Ordering::Greater),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceId {
    component_version: ComponentVersion,
    component_name: Option<ComponentName>,
    module_name: Option<ModuleName>,
    family: Option<Families>,
    relative_file: RelativeFile,
    display_simplified: bool,
}

impl ResourceId {
    pub fn new(relative_file: impl Into<RelativeFile>) -> Self {
        Self {
            component_version: ComponentVersion::empty(),
            component_name: None,
            module_name: None,
            family: None,
            relative_file: relative_file.into(),
            display_simplified: false,
        }
    }

    pub fn with_family(mut self, family: Families) -> Self {
        self.family = Some(family);
        self
    }

    pub fn with_module(mut self, module_name: impl Into<ModuleName>) -> Self {
        self.module_name = Some(module_name.into());
        self
    }

    pub fn with_component_name(mut self, component_name: impl Into<ComponentName>) -> Self {
        self.component_name = Some(component_name.into());
        self
    }

    pub fn with_component_version(
        mut self,
        component_version: impl Into<ComponentVersion>,
    ) -> Self {
        self.component_version = component_version.into();
        self
    }

    pub fn module(&self) -> Option<&ModuleName> {
        self.module_name.as_ref()
    }

    pub fn relative_file(&self) -> &RelativeFile {
        &self.relative_file
    }

    pub fn simplify(&mut self, display_simplified: bool) -> &mut Self {
        self.display_simplified = display_simplified;
        self
    }

    pub fn create_simplified(&self, detail_level: ResourceIdDetailLevel) -> ResourceId {
        let mut new_id = ResourceId::new(self.relative_file.clone());
        new_id.display_simplified = true;

        if detail_level > ResourceIdDetailLevel::RelativeFile
            && let Some(module_name) = &self.module_name
        {
            new_id = new_id.with_module(module_name.clone());
        } else {
            return new_id;
        }

        if detail_level > ResourceIdDetailLevel::Module
            && let Some(component_name) = &self.component_name
        {
            new_id = new_id.with_component_name(component_name.clone());
        } else {
            return new_id;
        }

        if detail_level > ResourceIdDetailLevel::ComponentName && !self.component_version.is_empty()
        {
            new_id = new_id.with_component_version(self.component_version.clone());
        }

        new_id
    }
}

impl Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut component_written = false;
        if let Some(component_name) = &self.component_name {
            if !self.component_version.is_empty() {
                write!(f, "{}@", self.component_version)?;
            }
            write!(f, "{component_name}:")?;
            component_written = true;
        }

        match (
            component_written,
            &self.module_name,
            self.display_simplified,
        ) {
            (true, Some(module_name), true) if module_name.as_ref() == ROOT_MODULE_NAME => {
                write!(f, ":")?
            }
            (_, Some(module_name), _) => write!(f, "{module_name}:")?,
            (true, None, true) => write!(f, ":")?,
            (true, None, false) => write!(f, "{ROOT_MODULE_NAME}:")?,
            _ => {}
        };

        match self.family {
            Some(Families::Attachments) => write!(f, "attachment$")?,
            Some(Families::Examples) => write!(f, "example$")?,
            Some(Families::Images) if self.display_simplified => write!(f, "")?,
            Some(Families::Images) => write!(f, "image$")?,
            Some(Families::Pages) if self.display_simplified => write!(f, "")?,
            Some(Families::Pages) => write!(f, "page$")?,
            Some(Families::Partials) => write!(f, "partial$")?,
            None => {}
        }

        write!(f, "{}", self.relative_file)
    }
}

impl TryFrom<&str> for ResourceId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut remainder = value;

        let component_version = if remainder.contains("@") {
            let mut split = remainder.split("@");
            let v = split.next().unwrap();
            remainder = split.next().unwrap();
            ComponentVersion::try_from(v.to_owned())?
        } else {
            ComponentVersion::empty()
        };

        let (component_name, module_name) = if remainder.contains("::") {
            let mut split = remainder.split("::");
            let component_name = split.next().unwrap();
            remainder = split.next().unwrap();
            (
                Some(ComponentName::try_from(component_name.to_owned())?),
                Some(ROOT_MODULE.clone()),
            )
        } else if !remainder.contains(":") {
            (None, None)
        } else {
            let mut split = remainder.split(":");
            let first = split.next().unwrap();
            let second = split.next().unwrap();
            if let Some(third) = split.next() {
                let component_name = ComponentName::try_from(first.to_owned())
                    .map_err(|_| Error::InvalidComponentName(HINT_COMPONENT_NAME.to_owned()))?;
                let module_name = ModuleName::try_from(second.to_owned())?;
                remainder = third;
                (Some(component_name), Some(module_name))
            } else {
                remainder = second;
                let module_name = ModuleName::try_from(first.to_owned())?;
                (None, Some(module_name))
            }
        };

        let family = if remainder.contains("$") {
            let mut split = remainder.split("$");
            let family = format!("{}s", split.next().unwrap());
            remainder = split.next().unwrap();
            match family {
                _ if family == ATTACHMENTS => Some(Families::Attachments),
                _ if family == EXAMPLES => Some(Families::Examples),
                _ if family == IMAGES => Some(Families::Images),
                _ if family == PAGES => Some(Families::Pages),
                _ if family == PARTIALS => Some(Families::Partials),
                _ => return Err(Error::InvalidFamily),
            }
        } else {
            None
        };

        let remainder = remainder.replace("\\", "/");
        let relative_file = if !remainder.contains("/") {
            create_relative_file_from_filename(
                Filename::try_from(remainder.to_owned())
                    .map_err(|_| Error::InvalidRelativePathToResource)?,
            )
        } else {
            let mut split = remainder.split("/");
            let mut relative_dir = create_relative_dir_from_dirname(
                Dirname::try_from(split.next().unwrap().to_owned())
                    .map_err(|_| Error::InvalidRelativePathToResource)?,
            );
            let entries = split.map(str::to_owned).collect::<Vec<String>>();
            for dir in &entries[..entries.len() - 1] {
                relative_dir.push_dir(
                    Dirname::try_from(dir.to_owned())
                        .map_err(|_| Error::InvalidRelativePathToResource)?,
                );
            }
            relative_dir.push_file(
                Filename::try_from(entries[entries.len() - 1].clone())
                    .map_err(|_| Error::InvalidRelativePathToResource)?,
            )
        };

        Ok(ResourceId {
            component_version,
            component_name,
            module_name,
            family,
            relative_file,
            display_simplified: false,
        })
    }
}

impl Serialize for ResourceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ResourceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ResourceId::try_from(s.as_str()).map_err(|e| DeError::custom(format!("{e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_deserialize_serialze {
        ($serialized: expr) => {{
            let resource_id = ResourceId::try_from($serialized).unwrap();
            assert!(resource_id.to_string() == $serialized);
        }};
        ($serialized_in: expr, $serialized_out:expr) => {{
            let resource_id = ResourceId::try_from($serialized_in).unwrap();
            assert!(resource_id.to_string() == $serialized_out);
        }};
    }

    #[test]
    fn resource_id_from_str_works() {
        test_deserialize_serialze!("relative/file.to");
        test_deserialize_serialze!("ROOT:relative/file.to");
        test_deserialize_serialze!("comp:mod:relative/file.to");
        test_deserialize_serialze!("v1.0@comp:my-mod:attachment$relative/file.to");
        test_deserialize_serialze!("comp:ROOT:example$relative/file.to");
        test_deserialize_serialze!("image$relative/file.to");
        test_deserialize_serialze!(
            "ne-compo::page$relative/file.to",
            "ne-compo:ROOT:page$relative/file.to"
        );
        test_deserialize_serialze!("comp:ROOT:partial$relative/file.to");
        test_deserialize_serialze!("comp::relative/file.to", "comp:ROOT:relative/file.to");
    }
}
