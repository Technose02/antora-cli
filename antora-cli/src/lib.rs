use init_task::TemplateResolver;

mod application;
pub mod tasks;

mod defaulttemplateresolver;
pub use defaulttemplateresolver::DefaultTemplateResolver;

pub fn run_from_env_args(template_resolver: Box<dyn TemplateResolver>) {
    application::run_cli(std::env::args_os(), template_resolver)
}

pub fn run_from_args<I, T, R>(args_iter: I, template_resolver: Box<dyn TemplateResolver>)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    application::run_cli(args_iter, template_resolver)
}

#[macro_export]
macro_rules! run_cli {
    ($template_resolver: ident) => {{ antora_cli::run_from_env_args($template_resolver) }};
    ($args_iter: ident, $template_resolver: ident) => {{ antora_cli::run_from_args($args_iter, $template_resolver) }};
}
