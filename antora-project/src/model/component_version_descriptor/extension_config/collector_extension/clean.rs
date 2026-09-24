use serde::{
    Deserialize, Serialize,
    de::{
        Visitor,
        value::{MapAccessDeserializer, SeqAccessDeserializer},
    },
};

#[derive(Debug, Clone)]
pub enum CleanValue {
    String(String),
    Array(Vec<CleanValueArrayEntry>),
    Map(CleanValueMap),
}

#[derive(Default)]
pub struct CleanValueArrayBuilder(Vec<CleanValueArrayEntry>);

#[allow(unused)]
impl CleanValueArrayBuilder {
    pub fn push(mut self, entry: impl Into<CleanValueArrayEntry>) -> Self {
        self.0.push(entry.into());
        self
    }
    pub fn build(self) -> CleanValue {
        CleanValue::Array(self.0)
    }
}

impl From<String> for CleanValue {
    fn from(value: String) -> Self {
        CleanValue::String(value)
    }
}

impl From<&str> for CleanValue {
    fn from(value: &str) -> Self {
        CleanValue::String(value.to_string())
    }
}

impl From<Vec<CleanValueArrayEntry>> for CleanValue {
    fn from(value: Vec<CleanValueArrayEntry>) -> Self {
        CleanValue::Array(value)
    }
}

impl From<CleanValueMap> for CleanValue {
    fn from(value: CleanValueMap) -> Self {
        CleanValue::Map(value)
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CleanValueMap {
    dir: String,
}

#[allow(unused)]
impl CleanValueMap {
    pub fn new(dir: String) -> Self {
        Self { dir }
    }
}

impl Serialize for CleanValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CleanValue::String(value) => value.serialize(serializer),
            CleanValue::Array(value) => value.serialize(serializer),
            CleanValue::Map(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CleanValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(CleanValueVisitor)
    }
}

struct CleanValueVisitor;

impl<'de> Visitor<'de> for CleanValueVisitor {
    type Value = CleanValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a String, an Array or a Map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CleanValue::String(v.into()))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let array = Deserialize::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(CleanValue::Array(array))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(CleanValue::Map(map))
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum CleanValueArrayEntry {
    String(String),
    Map(CleanValueMap),
}

impl Serialize for CleanValueArrayEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::String(value) => value.serialize(serializer),
            Self::Map(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CleanValueArrayEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(CleanValueArrayEntryVisitor)
    }
}

struct CleanValueArrayEntryVisitor;

impl<'de> Visitor<'de> for CleanValueArrayEntryVisitor {
    type Value = CleanValueArrayEntry;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a String or a Map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CleanValueArrayEntry::String(v.into()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(CleanValueArrayEntry::Map(map))
    }
}
