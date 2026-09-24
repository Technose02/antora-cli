use antora_fs::ANTORA_CACHE_DIR;
use antora_project::antora_playbook::AntoraPlaybook;
use std::fmt::Display;

mod confluence;
mod init;
mod site;
pub use confluence::run as run_confluence;
pub use init::run as run_init;
pub use site::run as run_site;

#[derive(Default, Clone)]
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

pub struct InitArgs<'cli> {
    pub(crate) project_dir: Option<&'cli String>,
    pub(crate) non_interactive_flag: bool,
    pub(crate) include_scaffolding: bool,
    pub(crate) provided_docs_dir: Option<&'cli String>,
    pub(crate) provided_component_name: Option<&'cli String>,
    pub(crate) provided_component_title: Option<&'cli String>,
    pub(crate) provided_component_version: Option<&'cli String>,
    pub(crate) provided_playbook_site_title: Option<&'cli String>,
    pub(crate) export_pdf: bool,
}

pub struct SiteArgs<'cli> {
    pub(crate) project_dir: Option<&'cli String>,
    pub(crate) playbook_filename: Option<&'cli String>,
    pub(crate) fetch: bool,
    pub(crate) stacktrace: bool,
    pub(crate) log_level: AntoraLogLevel,
    pub(crate) open: bool,
}

pub struct ConfluenceArgs<'cli> {
    pub(crate) project_dir: Option<&'cli String>,
    pub(crate) playbook: Option<&'cli String>,
    pub(crate) fetch: bool,
    pub(crate) stacktrace: bool,
    pub(crate) log_level: AntoraLogLevel,
}

fn get_antora_cache_dir(playbook: Option<&AntoraPlaybook>) -> String {
    playbook
        .and_then(|p| p.runtime.as_ref())
        .and_then(|r| r.cache_dir.as_ref())
        .map(String::as_str)
        .unwrap_or(ANTORA_CACHE_DIR)
        .into()
}
