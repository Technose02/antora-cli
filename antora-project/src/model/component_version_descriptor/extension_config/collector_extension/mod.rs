use serde::{
    Deserialize, Serialize,
    de::{
        Visitor,
        value::{MapAccessDeserializer, SeqAccessDeserializer},
    },
};

pub mod clean;
pub mod run;
pub mod scan;
use clean::CleanValue;
use run::RunValue;
use scan::ScanValue;

#[derive(Debug, Clone)]
pub enum CollectorExtensionConfig {
    Array(Vec<CollectorConfigArrayEntry>),
    Map(Box<CollectorConfigMap>),
}

#[derive(Default)]
pub struct CollectorConfigMapBuilder {
    // todo: worktree
    clean: Option<CleanValue>,
    run: Option<RunValue>,
    scan: Option<ScanValue>,
}

#[allow(unused)]
impl CollectorConfigMapBuilder {
    pub fn clean(mut self, value: CleanValue) -> Self {
        self.clean = Some(value);
        self
    }

    pub fn run(mut self, value: RunValue) -> Self {
        self.run = Some(value);
        self
    }

    pub fn scan(mut self, value: ScanValue) -> Self {
        self.scan = Some(value);
        self
    }

    pub fn build(self) -> CollectorExtensionConfig {
        CollectorExtensionConfig::Map(Box::new(CollectorConfigMap {
            clean: self.clean,
            run: self.run,
            scan: self.scan,
        }))
    }
}

#[derive(Default)]
pub struct CollectorConfigArrayBuilder(Vec<CollectorConfigArrayEntry>);

#[allow(unused)]
impl CollectorConfigArrayBuilder {
    pub fn clean(mut self, value: impl Into<CleanValue>) -> Self {
        self.0.push(CollectorConfigArrayEntry::Clean(value.into()));
        self
    }

    pub fn run(mut self, value: RunValue) -> Self {
        self.0.push(CollectorConfigArrayEntry::Run(value));
        self
    }

    pub fn scan(mut self, value: ScanValue) -> Self {
        self.0.push(CollectorConfigArrayEntry::Scan(value));
        self
    }

    pub fn build(self) -> CollectorExtensionConfig {
        CollectorExtensionConfig::Array(self.0)
    }
}

impl Serialize for CollectorExtensionConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CollectorExtensionConfig::Array(array) => array.serialize(serializer),
            CollectorExtensionConfig::Map(map) => map.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CollectorExtensionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(CollectorExtensionConfigVisitor)
    }
}

struct CollectorExtensionConfigVisitor;
impl<'de> Visitor<'de> for CollectorExtensionConfigVisitor {
    type Value = CollectorExtensionConfig;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Vec<CollectorConfigArrayEntry>) or a CollectorConfigMap")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let array = Deserialize::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(CollectorExtensionConfig::Array(array))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(CollectorExtensionConfig::Map(Box::new(map)))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectorConfigMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean: Option<CleanValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<RunValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan: Option<ScanValue>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
#[allow(unused)]
pub enum CollectorConfigArrayEntry {
    // TODO: Worktree
    Clean(CleanValue),
    Run(RunValue),
    Scan(ScanValue),
}
