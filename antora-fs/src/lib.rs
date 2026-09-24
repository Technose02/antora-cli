mod check;
mod error;
mod helpers;
mod model;

pub const ANTORA_CONFIGURATION: &str = "antora.toml";
pub const ANTORA_SECRETS_CONFIGURATION: &str = "antora-secrets.toml";
pub const DEFAULT_CONTENT_SOURCE_ROOT: &str = "docs";
const COMPONENT_VERSION_DESCRIPTOR: &str = "antora.yml";
const GITIGNORE: &str = ".gitignore";
pub const ANTORA_CACHE_DIR: &str = ".antora-cache";
pub const ANTORA_BUILD_DIR: &str = "antora-build";

pub use error::{Error, Result};
pub use helpers::read_antora_secrets_from_dir;
pub use model::{ComponentHandle, ModuleHandle, NormalizePath, ProjectInitDir, Resource, Vfs};
