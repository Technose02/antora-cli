use antora_cli::{CliArgs, run_cli};
use clap::Parser;

mod defaulttemplateresolver;
use defaulttemplateresolver::DefaultTemplateResolver;

#[derive(Debug, Parser)]
#[command(name = "antora-cli", bin_name = "antora-cli", version, about)]
struct CliApp {
    #[command(flatten)]
    cli_args: CliArgs,
}

fn main() {
    let template_resolver = Box::new(DefaultTemplateResolver::default());
    run_cli(CliApp::parse().cli_args, template_resolver)
}
