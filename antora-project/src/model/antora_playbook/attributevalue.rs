use serde::{Deserialize, Serialize, de::Visitor};

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    String(String),
    Bool(bool),
    None,
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        AttributeValue::String(value)
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        AttributeValue::String(value.to_owned())
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        AttributeValue::Bool(value)
    }
}

impl Serialize for AttributeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AttributeValue::Bool(val) => serializer.serialize_bool(*val),
            AttributeValue::String(s) => serializer.serialize_str(s),
            AttributeValue::None => serializer.serialize_none(),
        }
    }
}

struct AttributeValueVisitor;

impl<'de> Visitor<'de> for AttributeValueVisitor {
    type Value = AttributeValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting a bool, a string or nothing")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AttributeValue::Bool(v))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AttributeValue::None)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AttributeValue::String(v.into()))
    }
}

impl<'de> Deserialize<'de> for AttributeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(AttributeValueVisitor)
    }
}
