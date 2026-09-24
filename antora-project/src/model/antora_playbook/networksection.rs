use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct NetworkSection {
    pub no_proxy: String,
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct NetworkSectionBuilder {
    value: NetworkSection,
}

#[expect(unused)]
impl NetworkSectionBuilder {
    pub fn new() -> Self {
        NetworkSectionBuilder {
            value: NetworkSection::default(),
        }
    }

    pub fn init(network_section: NetworkSection) -> Self {
        NetworkSectionBuilder {
            value: network_section,
        }
    }

    pub fn no_proxy(mut self, no_proxy: impl Into<String>) -> Self {
        self.value.no_proxy = no_proxy.into();
        self
    }

    pub fn build(self) -> NetworkSection {
        self.value
    }
}

// endregion:   --- Builder(s)
