use crate::{DEFAULT_CONFLUENCE_PUBLISH_PLAYBOOK_FILENAME, Error};
use relative_path::Filename;
use serde::{Deserialize, Serialize, de::Error as DeError};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfluenceConfig {
    pub playbook: Filename,
    pub asciidoc_root_folder: String,
    pub root_confluence_url: String,
    pub space_key: String,
    pub ancestor_id: String,
    pub publisher_version: String,
    pub skip_ssl_verification: bool,
    pub max_requests_per_second: usize,
    pub publishing_strategy: PublishingStrategy,
    pub orphan_removal_strategy: OrphanRemovalStrategy,
    pub rest_api_version: String,
    pub notify_watchers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_title_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_title_suffix: Option<String>,
    pub projectversion: String,
}

impl Default for ConfluenceConfig {
    fn default() -> Self {
        Self {
            playbook: Filename::try_from(DEFAULT_CONFLUENCE_PUBLISH_PLAYBOOK_FILENAME)
                .expect("Filename::try_from must succeed for a valid filename"),
            asciidoc_root_folder: "<put the folder containing the asciidoc-root-document here (antora-build/vanilladoc-lite/...)>".to_owned(),
            root_confluence_url: "<put the base-url of your confluence-instance here>".to_owned(),
            space_key: "<put the destination spacekey here>".to_owned(),
            ancestor_id: "<put the destination ancestor-id here>".to_owned(),
            publishing_strategy: PublishingStrategy::AppendToAncestor,
            orphan_removal_strategy: OrphanRemovalStrategy::Keep,
            publisher_version: "0.32.0".to_owned(),
            skip_ssl_verification: false,
            max_requests_per_second: 10,
            notify_watchers: false,
            rest_api_version: "v1".to_owned(),
            page_title_prefix: None,
            page_title_suffix: None,
            projectversion: "unused".to_owned(),
        }
    }
}

#[derive(Default, Debug)]
pub enum PublishingStrategy {
    #[default]
    AppendToAncestor,
    ReplaceAncestor,
}

impl Display for PublishingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PublishingStrategy::AppendToAncestor => write!(f, "APPEND_TO_ANCESTOR"),
            PublishingStrategy::ReplaceAncestor => write!(f, "REPLACE_ANCESTOR"),
        }
    }
}

impl TryFrom<&str> for PublishingStrategy {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "APPEND_TO_ANCESTOR" => Ok(PublishingStrategy::AppendToAncestor),
            "REPLACE_ANCESTOR" => Ok(PublishingStrategy::ReplaceAncestor),
            _ => Err(Error::NotAPublishingStrategyVariant(value.to_owned())),
        }
    }
}

impl<'de> Deserialize<'de> for PublishingStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PublishingStrategy::try_from(s.as_str()).map_err(|e| DeError::custom(format!("{e}")))
    }
}

impl Serialize for PublishingStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[derive(Default, Debug)]
pub enum OrphanRemovalStrategy {
    #[default]
    Keep,
    Remove,
}

impl Display for OrphanRemovalStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrphanRemovalStrategy::Keep => write!(f, "KEEP_ORPHANS"),
            OrphanRemovalStrategy::Remove => write!(f, "REMOVE_ORPHANS"),
        }
    }
}

impl TryFrom<&str> for OrphanRemovalStrategy {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "KEEP_ORPHANS" => Ok(OrphanRemovalStrategy::Keep),
            "REMOVE_ORPHANS" => Ok(OrphanRemovalStrategy::Remove),
            _ => Err(Error::NotAnOrphanRemovalStrategyVariant(value.to_owned())),
        }
    }
}

impl<'de> Deserialize<'de> for OrphanRemovalStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OrphanRemovalStrategy::try_from(s.as_str()).map_err(|e| DeError::custom(format!("{e}")))
    }
}

impl Serialize for OrphanRemovalStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
