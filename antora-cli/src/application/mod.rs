use std::{borrow::Borrow, ffi::OsString};

use crate::tasks::{ConfluenceArgs, SiteArgs, run_confluence, run_site};
use clap::Parser;
use init_task::{InitArgs, InitTask, TemplateResolver};
mod cli;

pub use cli::{CliApp, CliCommands};

pub fn run_cli<I, T>(args_iter: I, template_resolver: Box<dyn TemplateResolver>)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = CliApp::parse_from(args_iter);
    let project_dir = Into::<&Option<String>>::into(cli.project_dir.borrow()).as_ref();

    match &cli.command {
        CliCommands::Init {
            non_interactive,
            scaffolding,
            content_source_root,
            component_name,
            component_title,
            component_version,
            playbook_site_title,
            template_key,
            export_pdf,
        } => InitTask::run(
            InitArgs {
                project_dir,
                non_interactive_flag: *non_interactive,
                include_scaffolding: *scaffolding,
                provided_docs_dir: content_source_root.as_ref(),
                provided_component_name: component_name.as_ref(),
                provided_component_title: component_title.as_ref(),
                provided_component_version: component_version.as_ref(),
                provided_playbook_site_title: playbook_site_title.as_ref(),
                template_key: template_key.as_ref(),
                export_pdf: *export_pdf,
            },
            template_resolver,
        ),
        CliCommands::Site {
            playbook,
            fetch,
            stacktrace,
            log_level,
            open,
        } => run_site(SiteArgs {
            project_dir,
            playbook_filename: playbook.as_ref(),
            fetch: *fetch,
            stacktrace: *stacktrace,
            log_level: log_level.clone().unwrap_or_default(),
            open: *open,
        }),
        CliCommands::Confluence {
            playbook,
            fetch,
            stacktrace,
            log_level,
        } => run_confluence(ConfluenceArgs {
            project_dir,
            playbook: playbook.as_ref(),
            fetch: *fetch,
            stacktrace: *stacktrace,
            log_level: log_level.clone().unwrap_or_default(),
        }),
    }
}
