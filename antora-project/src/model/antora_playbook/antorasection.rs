use relative_path::RelativeFile;
use serde::{
    Deserialize, Serialize,
    de::{Visitor, value::MapAccessDeserializer},
};
use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AntoraExtensionModule(String);

impl From<&str> for AntoraExtensionModule {
    fn from(value: &str) -> Self {
        AntoraExtensionModule(value.to_string())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Clone, Deserialize)]
pub struct AntoraExtensionMap {
    pub require: AntoraExtensionModule,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file: Option<RelativeFile>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum AntoraExtension {
    Module(AntoraExtensionModule),
    Map(AntoraExtensionMap),
}

impl From<AntoraExtensionModule> for AntoraExtension {
    fn from(value: AntoraExtensionModule) -> Self {
        AntoraExtension::Module(value)
    }
}

impl From<AntoraExtensionMap> for AntoraExtension {
    fn from(value: AntoraExtensionMap) -> Self {
        AntoraExtension::Map(value)
    }
}

impl Serialize for AntoraExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AntoraExtension::Module(module) => module.serialize(serializer),
            AntoraExtension::Map(map) => map.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for AntoraExtension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(AntoraExtensionVisitor)
    }
}

struct AntoraExtensionVisitor;

impl<'de> Visitor<'de> for AntoraExtensionVisitor {
    type Value = AntoraExtension;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a String or a Map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AntoraExtension::Module(AntoraExtensionModule(v.into())))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(AntoraExtension::Map(map))
    }
}

pub struct AntoraExtensionBuilder {
    module: AntoraExtensionModule,
    config_file: Option<RelativeFile>,
    require: bool,
}

impl AntoraExtensionBuilder {
    pub fn new(module: impl Into<String>) -> Self {
        AntoraExtensionBuilder {
            module: AntoraExtensionModule(module.into()),
            config_file: None,
            require: false,
        }
    }

    pub fn require(mut self) -> Self {
        self.require = true;
        self
    }

    pub fn config_file(mut self, config_file: RelativeFile) -> Self {
        self.config_file = Some(config_file);
        self.require = true;
        self
    }

    pub fn build(self) -> AntoraExtension {
        match (self.config_file, self.require) {
            (Some(config_file), _) => AntoraExtension::Map(AntoraExtensionMap {
                config_file: Some(config_file),
                require: self.module,
            }),
            (None, true) => AntoraExtension::Map(AntoraExtensionMap {
                require: self.module,
                config_file: None,
            }),
            (None, false) => AntoraExtension::Module(self.module),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct AntoraSection {
    pub extensions: HashSet<AntoraExtension>,
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct AntoraSectionBuilder {
    value: AntoraSection,
}

impl AntoraSectionBuilder {
    pub fn new() -> Self {
        AntoraSectionBuilder {
            value: AntoraSection::default(),
        }
    }

    pub fn init(antora_section: AntoraSection) -> Self {
        AntoraSectionBuilder {
            value: antora_section,
        }
    }

    pub fn extension(mut self, extension: AntoraExtension) -> Self {
        self.value.extensions.insert(extension);
        self
    }

    pub fn extensions(mut self, extensions: impl Into<HashSet<AntoraExtension>>) -> Self {
        self.value.extensions = extensions.into();
        self
    }

    pub fn build(self) -> AntoraSection {
        self.value
    }
}

// endregion:   --- Builder(s)
