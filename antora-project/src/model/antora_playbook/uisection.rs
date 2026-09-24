use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct UiBundle {
    pub url: String,
    pub snapshot: bool,
}

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct UiSection {
    pub bundle: UiBundle,
    pub supplemental_files: String,
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct UiBundleBuilder {
    value: UiBundle,
}

impl UiBundleBuilder {
    pub fn new() -> Self {
        UiBundleBuilder {
            value: UiBundle::default(),
        }
    }

    pub fn init(ui_bundle: UiBundle) -> Self {
        UiBundleBuilder { value: ui_bundle }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.value.url = url.into();
        self
    }

    pub fn snapshot(mut self, snapshot: bool) -> Self {
        self.value.snapshot = snapshot;
        self
    }

    pub fn build(self) -> UiBundle {
        self.value
    }
}

#[derive(Default)]
pub struct UiSectionBuilder {
    value: UiSection,
}

impl UiSectionBuilder {
    pub fn new() -> Self {
        UiSectionBuilder {
            value: UiSection::default(),
        }
    }

    pub fn init(ui_section: UiSection) -> Self {
        UiSectionBuilder { value: ui_section }
    }

    pub fn bundle(mut self, bundle: impl Into<UiBundle>) -> Self {
        self.value.bundle = bundle.into();
        self
    }

    pub fn supplemental_files(mut self, supplemental_files: impl Into<String>) -> Self {
        self.value.supplemental_files = supplemental_files.into();
        self
    }

    pub fn build(self) -> UiSection {
        self.value
    }
}

// endregion:   --- Builder(s)
