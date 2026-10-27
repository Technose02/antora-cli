use std::{
    borrow::Borrow,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use crate::{
    application::cli::StructurizrCommands,
    tasks::{
        BuildArgs, ConfluenceArgs, run_build, run_confluence, run_export_structurizr_views,
        run_structurizr_local,
    },
};
use init_task::{InitArgs, InitTask, TemplateResolver};
mod cli;

pub use cli::{CliArgs, CliCommands};

pub fn run_cli(cli_args: CliArgs, template_resolver: Box<dyn TemplateResolver>) {
    let container_cleanup_command_target = Arc::new(Mutex::new(None::<Command>));

    let container_cleanup_command_target_cloned = container_cleanup_command_target.clone();
    ctrlc::set_handler(move || {
        let mut lock = container_cleanup_command_target_cloned.lock();
        let container_cleanup_command_target = lock
            .as_deref_mut()
            .expect("failed to lock on container_cleanup_map")
            .as_mut();
        if let Some(container_cleanup_command) = container_cleanup_command_target {
            let mut child = container_cleanup_command
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("error running container_cleanup_command");
            child
                .wait()
                .expect("error waiting for spawned container_cleanup");
        }
        std::process::exit(0);
    })
    .unwrap_or_else(|e| panic!("error registering signal-handler: {e}"));

    let project_dir = Into::<&Option<String>>::into(cli_args.project_dir.borrow()).as_ref();

    match &cli_args.command {
        CliCommands::Init {
            non_interactive,
            content_source_root,
            component_name,
            component_title,
            component_version,
            playbook_site_title,
            init_template_key,
            no_pdf_export,
        } => InitTask::run(
            InitArgs {
                project_dir,
                non_interactive_flag: *non_interactive,
                provided_docs_dir: content_source_root.as_ref(),
                provided_component_name: component_name.as_ref(),
                provided_component_title: component_title.as_ref(),
                provided_component_version: component_version.as_ref(),
                provided_playbook_site_title: playbook_site_title.as_ref(),
                provided_init_template_key: init_template_key.as_ref(),
                pdf_export: !(*no_pdf_export),
            },
            template_resolver,
        ),
        CliCommands::Build {
            playbook,
            fetch,
            stacktrace,
            log_level,
            open,
        } => run_build(
            BuildArgs {
                project_dir,
                playbook_filename: playbook.as_ref(),
                fetch: *fetch,
                stacktrace: *stacktrace,
                default_image_config: template_resolver.default_image_config(),
                log_level: log_level.clone().unwrap_or_default(),
                open: *open,
            },
            container_cleanup_command_target,
        ),
        CliCommands::Confluence {
            playbook,
            fetch,
            stacktrace,
            log_level,
        } => run_confluence(
            ConfluenceArgs {
                project_dir,
                playbook: playbook.as_ref(),
                fetch: *fetch,
                stacktrace: *stacktrace,
                default_image_config: template_resolver.default_image_config(),
                log_level: log_level.clone().unwrap_or_default(),
            },
            container_cleanup_command_target,
        ),
        CliCommands::Structurizr { command } => match command {
            StructurizrCommands::Local { host_port } => run_structurizr_local(
                project_dir,
                template_resolver.default_image_config(),
                Some(*host_port),
                container_cleanup_command_target,
            ),
            StructurizrCommands::ExportViews { format } => run_export_structurizr_views(
                project_dir,
                template_resolver.default_image_config(),
                Some(format),
                container_cleanup_command_target,
            ),
        },
    }
}
