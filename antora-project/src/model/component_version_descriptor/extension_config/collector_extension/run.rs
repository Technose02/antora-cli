use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{
        Error as DeError, Visitor,
        value::{MapAccessDeserializer, SeqAccessDeserializer},
    },
};

#[derive(Debug, Clone)]
pub enum RunValue {
    String(String),
    Array(Vec<RunValueArrayEntry>),
    Map(RunValueMap),
}

#[derive(Default)]
pub struct RunValueArrayBuilder(Vec<RunValueArrayEntry>);

#[allow(unused)]
impl RunValueArrayBuilder {
    pub fn push(mut self, entry: impl Into<RunValueArrayEntry>) -> Self {
        self.0.push(entry.into());
        self
    }
    pub fn build(self) -> RunValue {
        RunValue::Array(self.0)
    }
}

impl From<String> for RunValue {
    fn from(command: String) -> Self {
        RunValue::String(command)
    }
}

impl From<Vec<RunValueArrayEntry>> for RunValue {
    fn from(entries: Vec<RunValueArrayEntry>) -> Self {
        RunValue::Array(entries)
    }
}

impl From<RunValueMap> for RunValue {
    fn from(map: RunValueMap) -> Self {
        RunValue::Map(map)
    }
}

#[derive(Clone, Debug, Serialize, Default, Deserialize)]
pub struct EnvEntry {
    name: String,
    value: String,
}

#[allow(unused)]
impl EnvEntry {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum RunEnv {
    Array(Vec<EnvEntry>),
    Map(HashMap<String, String>),
}

impl Default for RunEnv {
    fn default() -> Self {
        RunEnv::Array(Vec::new())
    }
}

impl Serialize for RunEnv {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RunEnv::Array(entries) => entries.serialize(serializer),
            RunEnv::Map(map) => map.serialize(serializer),
        }
    }
}

struct RunEnvVisitor;

impl<'de> Visitor<'de> for RunEnvVisitor {
    type Value = RunEnv;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either an Array or a Map")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let array = Deserialize::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(RunEnv::Array(array))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(RunEnv::Map(map))
    }
}

impl<'de> Deserialize<'de> for RunEnv {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(RunEnvVisitor)
    }
}

#[derive(Clone, Debug, Default)]
#[allow(unused)]
pub enum RunFailure {
    Ignore,
    Log,
    #[default]
    Throw,
}

impl Serialize for RunFailure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RunFailure::Ignore => serializer.serialize_str("ignore"),
            RunFailure::Log => serializer.serialize_str("log"),
            RunFailure::Throw => serializer.serialize_str("throw"),
        }
    }
}

impl<'de> Deserialize<'de> for RunFailure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "ignore" => Ok(RunFailure::Ignore),
            "log" => Ok(RunFailure::Log),
            "throw" => Ok(RunFailure::Throw),
            _ => Err(DeError::custom(
                "RunFailure must be either 'ignore', 'log', or 'throw'",
            )),
        }
    }
}

#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct RunValueMap {
    command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shell: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<RunEnv>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure: Option<RunFailure>,
}

#[allow(unused)]
impl RunValueMap {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            ..Default::default()
        }
    }

    pub fn dir(mut self, dir: impl Into<String>) -> Self {
        self.dir = Some(dir.into());
        self
    }

    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }

    pub fn shell(mut self, shell: bool) -> Self {
        self.shell = Some(shell);
        self
    }
    pub fn env(mut self, env: RunEnv) -> Self {
        self.env = Some(env);
        self
    }

    pub fn failure(mut self, failure: RunFailure) -> Self {
        self.failure = Some(failure);
        self
    }
}

impl Serialize for RunValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RunValue::String(command) => command.serialize(serializer),
            RunValue::Array(entries) => entries.serialize(serializer),
            RunValue::Map(map) => map.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for RunValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(RunValueVisitor)
    }
}

struct RunValueVisitor;

impl<'de> Visitor<'de> for RunValueVisitor {
    type Value = RunValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a String, an Array or a Map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RunValue::String(v.into()))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let array = Deserialize::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(RunValue::Array(array))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(RunValue::Map(map))
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum RunValueArrayEntry {
    String(String),
    Map(RunValueMap),
}

impl From<RunValueMap> for RunValueArrayEntry {
    fn from(value: RunValueMap) -> Self {
        RunValueArrayEntry::Map(value)
    }
}

impl Serialize for RunValueArrayEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::String(command) => command.serialize(serializer),
            Self::Map(map) => map.serialize(serializer),
        }
    }
}

struct RunValueArrayEntryVisitor;

impl<'de> Visitor<'de> for RunValueArrayEntryVisitor {
    type Value = RunValueArrayEntry;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a String or a Map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RunValueArrayEntry::String(v.into()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let map = Deserialize::deserialize(MapAccessDeserializer::new(map))?;
        Ok(RunValueArrayEntry::Map(map))
    }
}

impl<'de> Deserialize<'de> for RunValueArrayEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(RunValueArrayEntryVisitor)
    }
}
