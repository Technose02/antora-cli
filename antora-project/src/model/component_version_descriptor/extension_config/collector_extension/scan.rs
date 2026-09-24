use serde::{
    Deserialize, Serialize,
    de::{
        Visitor,
        value::{MapAccessDeserializer, SeqAccessDeserializer},
    },
};

#[derive(Debug, Clone)]
pub enum ScanValue {
    String(String),
    Array(Vec<ScanValueArrayEntry>),
    Map(ScanValueMap),
}

#[derive(Default)]
pub struct ScanValueArrayBuilder(Vec<ScanValueArrayEntry>);

#[allow(unused)]
impl ScanValueArrayBuilder {
    pub fn push(mut self, entry: impl Into<ScanValueArrayEntry>) -> Self {
        self.0.push(entry.into());
        self
    }
    pub fn build(self) -> ScanValue {
        ScanValue::Array(self.0)
    }
}

impl From<String> for ScanValue {
    fn from(dir: String) -> Self {
        ScanValue::String(dir)
    }
}

impl From<Vec<ScanValueArrayEntry>> for ScanValue {
    fn from(entries: Vec<ScanValueArrayEntry>) -> Self {
        ScanValue::Array(entries)
    }
}

impl From<ScanValueMap> for ScanValue {
    fn from(map: ScanValueMap) -> Self {
        ScanValue::Map(map)
    }
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct ScanIntoMap {
    name: String,
    version: String,
    dir: String,
}

#[allow(unused)]
impl ScanIntoMap {
    pub fn new(name: String, version: String, dir: String) -> Self {
        Self { name, version, dir }
    }
}

impl From<String> for ScanInto {
    fn from(value: String) -> Self {
        ScanInto::BasePath(value)
    }
}

impl From<&str> for ScanInto {
    fn from(value: &str) -> Self {
        ScanInto::BasePath(value.to_string())
    }
}

impl From<ScanIntoMap> for ScanInto {
    fn from(value: ScanIntoMap) -> Self {
        ScanInto::Map(value)
    }
}

#[derive(Debug, Clone)]
pub enum ScanInto {
    BasePath(String),
    Map(ScanIntoMap),
}

impl Serialize for ScanInto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ScanInto::BasePath(value) => value.serialize(serializer),
            ScanInto::Map(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ScanInto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ScanIntoVisitor)
    }
}

struct ScanIntoVisitor;

impl<'de> Visitor<'de> for ScanIntoVisitor {
    type Value = ScanInto;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a String or a Map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ScanInto::BasePath(v.into()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(ScanInto::Map(map))
    }
}

#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct ScanValueMap {
    dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    into: Option<ScanInto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clean: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
}

#[allow(unused)]
impl ScanValueMap {
    pub fn new(dir: impl Into<String>) -> Self {
        Self {
            dir: dir.into(),
            ..Default::default()
        }
    }

    pub fn files(mut self, files: impl Into<String>) -> Self {
        self.files = Some(files.into());
        self
    }

    pub fn into(mut self, into: impl Into<ScanInto>) -> Self {
        self.into = Some(into.into());
        self
    }

    pub fn clean(mut self, clean: bool) -> Self {
        self.clean = Some(clean);
        self
    }
    pub fn private(mut self, private: bool) -> Self {
        self.private = Some(private);
        self
    }
}

impl Serialize for ScanValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ScanValue::String(dir) => dir.serialize(serializer),
            ScanValue::Array(entries) => entries.serialize(serializer),
            ScanValue::Map(map) => map.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ScanValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ScanValueVisitor)
    }
}

struct ScanValueVisitor;

impl<'de> Visitor<'de> for ScanValueVisitor {
    type Value = ScanValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a String, an Array or a Map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ScanValue::String(v.into()))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let array = Deserialize::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(ScanValue::Array(array))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(ScanValue::Map(map))
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum ScanValueArrayEntry {
    String(String),
    Map(ScanValueMap),
}

impl From<ScanValueMap> for ScanValueArrayEntry {
    fn from(value: ScanValueMap) -> Self {
        ScanValueArrayEntry::Map(value)
    }
}

impl Serialize for ScanValueArrayEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::String(dir) => dir.serialize(serializer),
            Self::Map(map) => map.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ScanValueArrayEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ScanValueArrayEntryVisitor)
    }
}

struct ScanValueArrayEntryVisitor;

impl<'de> Visitor<'de> for ScanValueArrayEntryVisitor {
    type Value = ScanValueArrayEntry;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a String or a Map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ScanValueArrayEntry::String(v.into()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(ScanValueArrayEntry::Map(map))
    }
}
