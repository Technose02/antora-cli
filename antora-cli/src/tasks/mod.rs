use antora_fs::ANTORA_CACHE_DIR;
use antora_project::{antora_configuration::ImageConfig, antora_playbook::AntoraPlaybook};
use std::fmt::Display;

mod confluence;
//mod init;
mod site;
mod structurizr;
pub use confluence::run as run_confluence;
pub use site::run as run_site;

pub use structurizr::{
    STRUCTURIZR_CONTAINER_PORT, run_export_diagrams as run_export_structurizr_diagrams,
    run_local as run_structurizr_local,
};

#[derive(Debug, Default, Clone)]
pub enum AntoraLogLevel {
    Fatal,
    Error,
    #[default]
    Warn,
    Info,
    Debug,
    All,
    Silent,
}

impl Display for AntoraLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntoraLogLevel::Fatal => write!(f, "fatal"),
            AntoraLogLevel::Error => write!(f, "error"),
            AntoraLogLevel::Warn => write!(f, "warn"),
            AntoraLogLevel::Info => write!(f, "info"),
            AntoraLogLevel::Debug => write!(f, "debug"),
            AntoraLogLevel::All => write!(f, "all"),
            AntoraLogLevel::Silent => write!(f, "silent"),
        }
    }
}

impl From<&str> for AntoraLogLevel {
    fn from(value: &str) -> Self {
        match value {
            "fatal" => AntoraLogLevel::Fatal,
            "error" => AntoraLogLevel::Error,
            "warn" => AntoraLogLevel::Warn,
            "info" => AntoraLogLevel::Info,
            "debug" => AntoraLogLevel::Debug,
            "all" => AntoraLogLevel::All,
            "silent" => AntoraLogLevel::Silent,
            other => {
                let ret = AntoraLogLevel::default();
                eprintln!("invalid antora-loglevel '{other}'; using default ('{ret}')");
                ret
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ExportFormat {
    #[default]
    Png,
    Svg,
}

impl AsRef<str> for ExportFormat {
    fn as_ref(&self) -> &str {
        match self {
            ExportFormat::Png => "png",
            ExportFormat::Svg => "svg",
        }
    }
}

impl Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl From<&str> for ExportFormat {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "png" => ExportFormat::Png,
            "svg" => ExportFormat::Svg,
            other => {
                let ret = ExportFormat::default();
                eprintln!("invalid export-format '{other}'; using default ('{ret}')");
                ret
            }
        }
    }
}

pub struct SiteArgs<'cli> {
    pub(crate) project_dir: Option<&'cli String>,
    pub(crate) playbook_filename: Option<&'cli String>,
    pub(crate) fetch: bool,
    pub(crate) stacktrace: bool,
    pub(crate) log_level: AntoraLogLevel,
    pub(crate) default_image_config: ImageConfig,
    pub(crate) open: bool,
}

pub struct ConfluenceArgs<'cli> {
    pub(crate) project_dir: Option<&'cli String>,
    pub(crate) playbook: Option<&'cli String>,
    pub(crate) fetch: bool,
    pub(crate) stacktrace: bool,
    pub(crate) log_level: AntoraLogLevel,
    pub(crate) default_image_config: ImageConfig,
}

fn get_antora_cache_dir(playbook: Option<&AntoraPlaybook>) -> String {
    playbook
        .and_then(|p| p.runtime.as_ref())
        .and_then(|r| r.cache_dir.as_ref())
        .map(String::as_str)
        .unwrap_or(ANTORA_CACHE_DIR)
        .into()
}
