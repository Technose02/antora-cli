use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::AttributeValue;

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct AsciidocSection {
    pub extensions: HashSet<String>,
    pub attributes: HashMap<String, AttributeValue>,
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct AsciidocSectionBuilder {
    value: AsciidocSection,
}

impl AsciidocSectionBuilder {
    pub fn new() -> Self {
        AsciidocSectionBuilder {
            value: AsciidocSection::default(),
        }
    }

    pub fn init(asciidoc_section: AsciidocSection) -> Self {
        AsciidocSectionBuilder {
            value: asciidoc_section,
        }
    }

    pub fn extension(mut self, extension: impl Into<String>) -> Self {
        self.value.extensions.insert(extension.into());
        self
    }

    pub fn extensions(mut self, extensions: impl Into<HashSet<String>>) -> Self {
        self.value.extensions = extensions.into();
        self
    }

    pub fn attribute(
        mut self,
        attribute_name: impl Into<String>,
        attribute_value: impl Into<AttributeValue>,
    ) -> Self {
        self.value
            .attributes
            .insert(attribute_name.into(), attribute_value.into());
        self
    }

    pub fn attributes(mut self, attributes: impl Into<HashMap<String, AttributeValue>>) -> Self {
        self.value.attributes = attributes.into();
        self
    }

    pub fn build(self) -> AsciidocSection {
        self.value
    }
}

// endregion:   --- Builder(s)
