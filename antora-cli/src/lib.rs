mod application;
pub mod tasks;

pub fn run_from_env_args() {
    application::run_cli(std::env::args_os())
}

pub fn run_from_args<I, T>(args_iter: I)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    application::run_cli(args_iter)
}

#[macro_export]
macro_rules! run_cli {
    () => {{ antora_cli::run_from_env_args() }};
    ($args_iter: ident) => {{ antora_cli::run_from_args($args_iter) }};
}
