use crate::model::{
    component_name::ComponentName, component_version::ComponentVersion, resource_id::ResourceId,
};
use relative_path::RelativeFile;
use serde::{Deserialize, Serialize};
use serde_yaml_bw::to_string as to_yaml;
use std::{collections::HashMap, fmt::Display};

pub mod extension_config;
use extension_config::ExtensionConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentVersionDescriptor {
    name: ComponentName,
    version: ComponentVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_page: Option<ResourceId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    nav: Vec<RelativeFile>,
    #[serde(default, skip_serializing_if = "Asciidoc::is_empty")]
    asciidoc: Asciidoc,
    #[serde(skip_serializing_if = "Option::is_none")]
    ext: Option<ExtensionConfig>,

    // irrelevant?
    #[serde(skip_serializing_if = "Option::is_none")]
    display_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prerelease: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
struct Asciidoc {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    attributes: HashMap<String, String>,
}

impl Asciidoc {
    #[allow(unused)]
    fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

impl ComponentVersionDescriptor {
    pub fn basic(name: ComponentName) -> Self {
        Self {
            name,
            version: ComponentVersion::empty(),
            title: None,
            start_page: None,
            nav: Vec::new(),
            asciidoc: Asciidoc::default(),
            display_version: None,
            prerelease: None,
            ext: None,
        }
    }

    pub fn with_title(&mut self, title: impl Into<String>) -> &mut Self {
        let title = title.into();
        if !title.is_empty() {
            self.title = Some(title)
        }
        self
    }

    pub fn with_version(&mut self, component_version: &ComponentVersion) -> &mut Self {
        self.version = component_version.clone();
        self
    }

    pub fn with_nav(&mut self, nav_file_path: RelativeFile) -> &mut Self {
        self.nav.push(nav_file_path);
        self
    }

    pub fn with_ext(&mut self, extension_config: impl Into<ExtensionConfig>) -> &mut Self {
        self.ext = Some(extension_config.into());
        self
    }

    // getter

    pub fn version(&self) -> &ComponentVersion {
        &self.version
    }
}

impl ComponentVersionDescriptor {
    pub fn name(&self) -> &ComponentName {
        &self.name
    }
}

impl Display for ComponentVersionDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = to_yaml(&self)
            .expect("yaml-serialization of ComponentVersionDescriptor must always work");

        // fix
        write!(f, "{}", serialized.replace("'~'", "~"))
    }
}

// todo: these tests...
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_deserializes_and_serializes() {
        let mut acs = ComponentVersionDescriptor::basic("my-name");

        assert!(acs.name == "my-name");
        let serialized = acs.to_str().unwrap();
        assert!(serialized.contains("name: my-name"));

        acs = ComponentVersionDescriptor::from_str(&serialized).unwrap();
        assert!(acs.name == "my-name");
    }

    #[test]
    fn version_deserializes_and_serializes() {
        let mut acs = ComponentVersionDescriptor::basic("my-name");

        assert!(acs.version.as_ref() == "~");
        let serialized = acs.to_str().unwrap();
        assert!(serialized.contains("version: ~"));

        acs = ComponentVersionDescriptor::from_str(&serialized).unwrap();
        assert!(acs.version.as_ref() == "~");

        acs.version = "v0.1.0".into();
        assert!(acs.version.as_ref() == "v0.1.0");
        let serialized = acs.to_str().unwrap();
        assert!(serialized.contains("version: v0.1.0"));

        acs = ComponentVersionDescriptor::from_str(&serialized).unwrap();
        assert!(acs.version.as_ref() == "v0.1.0");
    }

    #[test]
    fn title_deserializes_and_serializes() {
        let mut acs = ComponentVersionDescriptor::basic("my-name");
        assert!(acs.title.is_none());
        let serialized = acs.to_str().unwrap();
        assert!(!serialized.contains("title:"));

        acs = ComponentVersionDescriptor::from_str(&serialized).unwrap();
        assert!(acs.title.is_none());

        acs.title = Some("titel".into());
        let serialized = acs.to_str().unwrap();
        assert!(serialized.contains("title: titel"));

        acs = ComponentVersionDescriptor::from_str(&serialized).unwrap();
        if let Some(s) = acs.title {
            assert!(s == "titel")
        } else {
            assert!(false)
        }
    }

    #[test]
    fn start_page_deserializes_and_serializes() {
        let mut acs = AntoraContentSource::basic("my-name");
        assert!(acs.start_page.is_none());
        let serialized = acs.to_str().unwrap();
        assert!(!serialized.contains("start_page:"));

        acs = AntoraContentSource::from_str(&serialized).unwrap();
        assert!(acs.start_page.is_none());

        acs.start_page = Some("Startseite".into());
        let serialized = acs.to_str().unwrap();
        assert!(serialized.contains("start_page: Startseite"));

        acs = AntoraContentSource::from_str(&serialized).unwrap();
        if let Some(s) = acs.start_page {
            assert!(s == "Startseite")
        } else {
            assert!(false)
        }
    }

    #[test]
    fn nav_deserializes_and_serializes() {
        let mut acs = AntoraContentSource::basic("my-name");
        assert!(acs.nav.is_empty());
        let serialized = acs.to_str().unwrap();
        assert!(!serialized.contains("nav:"));

        acs = AntoraContentSource::from_str(&serialized).unwrap();
        assert!(acs.nav.is_empty());

        acs.nav.push("modules/ROOT/nav.adoc".into());
        let serialized = acs.to_str().unwrap();
        assert!(serialized.contains("nav:"));
        assert!(serialized.contains("- modules/ROOT/nav.adoc"));

        acs = AntoraContentSource::from_str(&serialized).unwrap();
        if let Some(n0) = acs.nav.iter().next() {
            assert!(n0 == "modules/ROOT/nav.adoc")
        } else {
            assert!(false)
        }
    }

    #[test]
    fn asciidoc_deserializes_and_serializes() {
        let mut acs: AntoraContentSource = AntoraContentSource::basic("my-name");
        let serialized = acs.to_str().unwrap();
        assert!(!serialized.contains("asciidoc:"));

        acs = AntoraContentSource::from_str(&serialized).unwrap();
        assert!(acs.asciidoc.is_empty());

        acs.asciidoc
            .attributes
            .insert("testkey".into(), "testvalue".into());
        let serialized = acs.to_str().unwrap();
        assert!(serialized.contains("asciidoc:"));
        assert!(serialized.contains("attributes:"));
        assert!(serialized.contains("testkey: testvalue"));

        acs = AntoraContentSource::from_str(&serialized).unwrap();
        assert!(acs.asciidoc.attributes.contains_key("testkey"));
        if let Some(v) = acs.asciidoc.attributes.get("testkey") {
            assert!(v == "testvalue");
        } else {
            assert!(false)
        }
    }
}
*/
