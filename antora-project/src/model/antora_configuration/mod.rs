use crate::{DEFAULT_PLAYBOOK_FILENAME, Error};
use relative_path::Filename;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use toml::{de::from_str as parse_toml, ser::to_string_pretty as write_toml};

mod confluence_config;
pub use confluence_config::{ConfluenceConfig, OrphanRemovalStrategy};

#[derive(Serialize, Deserialize)]
pub struct AntoraConfiguration {
    pub playbook: PlaybookConfig,
    pub antora_image: ImageConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confluence: Option<ConfluenceConfig>,
}

impl AntoraConfiguration {
    pub fn playbook_filename(&self) -> &Filename {
        &self.playbook.filename
    }

    pub fn antora_image(&self) -> String {
        format!(
            "{}:{}",
            self.antora_image.antora_image, self.antora_image.version_tag
        )
    }

    pub fn antora_image_base(&self) -> &str {
        &self.antora_image.antora_image
    }

    pub fn antora_image_version_tag(&self) -> &str {
        &self.antora_image.version_tag
    }
}

impl TryFrom<&str> for AntoraConfiguration {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_toml::<AntoraConfiguration>(value).map_err(Error::TomlDeserializationError)
    }
}

impl Display for AntoraConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            write_toml(&self).expect("toml-serialization of AntoraConfiguration must always work"),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlaybookConfig {
    pub filename: Filename,
}

impl Default for PlaybookConfig {
    fn default() -> Self {
        Self {
            filename: Filename::try_from(DEFAULT_PLAYBOOK_FILENAME)
                .expect("Filename::try_from must succeed for a valid filename"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ImageConfig {
    pub antora_image: String,
    pub version_tag: String,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            antora_image: "<ANTORA_IMAGE_PATH>".to_owned(),
            version_tag: "<ANTORA_IMAGE_VERSION_TAG>".to_owned(),
        }
    }
}
