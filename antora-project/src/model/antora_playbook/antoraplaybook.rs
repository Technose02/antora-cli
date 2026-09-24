use std::fmt::Display;

use super::{
    AntoraSection, AsciidocSection, ContentSection, GitSection, NetworkSection, OutputSection,
    RuntimeSection, SiteSection, UiSection,
};
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_yaml_bw::{from_str as from_yaml, to_string as to_yaml};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct AntoraPlaybook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<SiteSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<UiSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<NetworkSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<GitSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antora: Option<AntoraSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<OutputSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<RuntimeSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asciidoc: Option<AsciidocSection>,
}

impl Display for AntoraPlaybook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            to_yaml(&self).expect("yaml-serialization of AntoraPlaybook must always work")
        )
    }
}

impl TryFrom<&str> for AntoraPlaybook {
    type Error = Error;
    fn try_from(source: &str) -> std::result::Result<Self, Self::Error> {
        from_yaml(source).map_err(Error::YamlDeserializationError)
    }
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct AntoraPlaybookBuilder {
    value: AntoraPlaybook,
}

impl AntoraPlaybookBuilder {
    pub fn new() -> Self {
        AntoraPlaybookBuilder {
            value: AntoraPlaybook::default(),
        }
    }

    pub fn init(antora_playbook: AntoraPlaybook) -> Self {
        AntoraPlaybookBuilder {
            value: antora_playbook,
        }
    }

    pub fn site(mut self, site: impl Into<SiteSection>) -> Self {
        self.value.site = Some(site.into());
        self
    }

    pub fn content(mut self, content: impl Into<ContentSection>) -> Self {
        self.value.content = Some(content.into());
        self
    }

    pub fn ui(mut self, ui: impl Into<UiSection>) -> Self {
        self.value.ui = Some(ui.into());
        self
    }

    pub fn git(mut self, git: impl Into<GitSection>) -> Self {
        self.value.git = Some(git.into());
        self
    }

    pub fn network(mut self, network: impl Into<NetworkSection>) -> Self {
        self.value.network = Some(network.into());
        self
    }

    pub fn output(mut self, output: impl Into<OutputSection>) -> Self {
        self.value.output = Some(output.into());
        self
    }

    pub fn runtime(mut self, runtime: impl Into<RuntimeSection>) -> Self {
        self.value.runtime = Some(runtime.into());
        self
    }

    pub fn antora(mut self, antora: impl Into<AntoraSection>) -> Self {
        self.value.antora = Some(antora.into());
        self
    }

    pub fn asciidoc(mut self, asciidoc: impl Into<AsciidocSection>) -> Self {
        self.value.asciidoc = Some(asciidoc.into());
        self
    }

    pub fn build(self) -> AntoraPlaybook {
        self.value
    }
}

// endregion:   --- Builder(s)
