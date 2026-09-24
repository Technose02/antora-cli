use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct OutputSection {
    pub dir: String,
    pub clean: bool,
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct OutputSectionBuilder {
    value: OutputSection,
}

impl OutputSectionBuilder {
    pub fn new() -> Self {
        OutputSectionBuilder {
            value: OutputSection::default(),
        }
    }

    pub fn init(output_section: OutputSection) -> Self {
        OutputSectionBuilder {
            value: output_section,
        }
    }

    pub fn dir(mut self, dir: impl Into<String>) -> Self {
        self.value.dir = dir.into();
        self
    }

    pub fn clean(mut self, clean: bool) -> Self {
        self.value.clean = clean;
        self
    }

    pub fn build(self) -> OutputSection {
        self.value
    }
}

// endregion:   --- Builder(s)
