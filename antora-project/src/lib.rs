mod error;
mod model;

pub const DEFAULT_PLAYBOOK_FILENAME: &str = "antora-playbook.yml";
pub const DEFAULT_CONFLUENCE_PUBLISH_PLAYBOOK_FILENAME: &str = "confluence-publish.yml";
pub const ROOT_MODULE_NAME: &str = "ROOT";

pub use error::{Error, Result};
pub use model::{
    antora_configuration, antora_playbook, antora_secrets, component_name, component_version,
    component_version_descriptor, families, module_name, resource_id,
};
