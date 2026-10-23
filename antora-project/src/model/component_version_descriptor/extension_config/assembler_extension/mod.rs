use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssemblerExtensionConfig {
    nav: Vec<String>,
}

impl AssemblerExtensionConfig {
    pub fn with_nav(mut self, nav: impl IntoIterator<Item = String>) -> Self {
        self.nav.extend(nav);
        self
    }
    pub fn with_nav_entry(mut self, nav_entry: impl Into<String>) -> Self {
        self.nav.push(nav_entry.into());
        self
    }
    pub fn nav(&self) -> &[String] {
        self.nav.as_slice()
    }
}
