use relative_path::RelativeDir;
use serde::{Deserialize, Serialize};
use serde_yaml_bw::FlowSeq;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
#[serde(untagged)]
pub enum Branches {
    #[default]
    None,
    Single(String),
    Multiple(FlowSeq<HashSet<String>>),
}

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Debug)]
pub struct ContentSource {
    pub url: String,
    pub branches: Branches,
    pub start_path: RelativeDir,
}

#[derive(Deserialize, Default, Serialize, Clone, PartialEq, Debug)]
pub struct ContentSection {
    pub sources: Vec<ContentSource>,
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct ContentSourceBuilder {
    value: ContentSource,
}

impl ContentSourceBuilder {
    pub fn new() -> Self {
        ContentSourceBuilder {
            value: ContentSource::default(),
        }
    }

    pub fn init(content_source: ContentSource) -> Self {
        ContentSourceBuilder {
            value: content_source,
        }
    }

    pub fn start_path(mut self, start_path: RelativeDir) -> Self {
        self.value.start_path = start_path;
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.value.url = url.into();
        self
    }

    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        let mut value = self.value.clone();
        match self.value.branches {
            Branches::None => value.branches = Branches::Single(branch.into()),
            Branches::Single(existing) => {
                let mut branches = HashSet::new();
                branches.insert(existing);
                branches.insert(branch.into());
                value.branches = Branches::Multiple(FlowSeq(branches));
            }
            Branches::Multiple(FlowSeq(existing_set)) => {
                let mut branches = HashSet::new();
                branches.extend(existing_set);
                branches.insert(branch.into());
                value.branches = Branches::Multiple(FlowSeq(branches));
            }
        }
        self.value = value;
        self
    }

    pub fn build(self) -> ContentSource {
        self.value
    }
}
// endregion:   --- ContentSourceBuilder

#[derive(Default)]
pub struct ContentSectionBuilder {
    value: ContentSection,
}

impl ContentSectionBuilder {
    pub fn new() -> Self {
        ContentSectionBuilder {
            value: ContentSection::default(),
        }
    }

    pub fn init(content_section: ContentSection) -> Self {
        ContentSectionBuilder {
            value: content_section,
        }
    }

    pub fn source(mut self, source: impl Into<ContentSource>) -> Self {
        self.value.sources.push(source.into());
        self
    }

    pub fn sources(mut self, sources: impl Into<Vec<ContentSource>>) -> Self {
        self.value.sources = sources.into();
        self
    }

    pub fn build(self) -> ContentSection {
        self.value
    }
}

// endregion:   --- Builder(s)
