mod application;
pub use application::{CliArgs, run_cli};
pub mod tasks;

mod defaulttemplateresolver;
pub use defaulttemplateresolver::DefaultTemplateResolver;

//pub fn run_from_env_args(template_resolver: Box<dyn TemplateResolver>) {
//    application::run_cli(std::env::args_os(), template_resolver)
//}

//pub fn run_cli(cli_args: application::CliArgs, template_resolver: Box<dyn TemplateResolver>) {
//    application::run_cli(cli_args, template_resolver)
//}

//#[macro_export]
//macro_rules! run_cli {
//    ($template_resolver: ident) => {{ antora_cli::run_from_env_args($template_resolver) }};
//    ($args_iter: ident, $template_resolver: ident) => {{ antora_cli::run_from_args($args_iter, $template_resolver) }};
//}
