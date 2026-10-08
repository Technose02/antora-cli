mod application;
pub use application::{CliArgs, run_cli};
pub mod tasks;

mod defaulttemplateresolver;
pub use defaulttemplateresolver::DefaultTemplateResolver;
