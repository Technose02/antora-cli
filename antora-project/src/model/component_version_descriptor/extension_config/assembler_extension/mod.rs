use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssemblerExtensionConfig {
    nav: Option<String>,
}

impl AssemblerExtensionConfig {
    pub fn with_nav(&mut self, nav: impl Into<String>) -> &mut Self {
        self.nav = Some(nav.into());
        self
    }
    pub fn nav(&self) -> Option<&String> {
        self.nav.as_ref()
    }
}
