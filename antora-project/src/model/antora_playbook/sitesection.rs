use crate::model::resource_id::ResourceId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct SiteSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_page: Option<ResourceId>,
    pub url: Option<String>,
}

// region:      --- Builder
#[derive(Default)]
pub struct SiteSectionBuilder {
    value: SiteSection,
}

impl SiteSectionBuilder {
    pub fn new() -> Self {
        SiteSectionBuilder {
            value: SiteSection::default(),
        }
    }

    pub fn init(site_section: SiteSection) -> Self {
        SiteSectionBuilder {
            value: site_section,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.value.title = Some(title.into());
        self
    }

    pub fn start_page(mut self, start_page: ResourceId) -> Self {
        self.value.start_page = Some(start_page);
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.value.url = Some(url.into());
        self
    }

    pub fn build(self) -> SiteSection {
        self.value
    }
}
// endregion:   --- Builder
