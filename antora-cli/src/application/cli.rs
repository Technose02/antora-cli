use crate::tasks::AntoraLogLevel;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "antora-cli helps you create, manage(todo!) and build your antora-projects", long_about = None)]
pub struct CliApp {
    /// the project-directory to be used in the specific task (default is current directory)
    #[arg(short = 'p', long, value_name = "DIR")]
    pub project_dir: Option<String>,

    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand)]
pub enum CliCommands {
    /// initializes a new project in the project-dir.
    /// This will create a new component and a default playbook using provided values or prompting for them.
    /// Existing files will not be overitten and no commits will be made to existing git-repositories.
    #[command(
        about = "initializes a new project in the project-dir",
        long_about = None
    )]
    Init {
        /// set to deactivate prompting for values not provided
        #[arg(short = 'n', long, value_name = "FLAG")]
        non_interactive: bool,

        /// set to include extra-contents on asciidoc and antora to help you getting started
        #[arg(short = 's', long, value_name = "FLAG")]
        scaffolding: bool,

        /// the content-source-root for your component (default: 'docs')
        #[arg(short = 'r', long, value_name = "DIR")]
        content_source_root: Option<String>,

        /// the name of your component (prompts for value in interactive-mode or error if not provided)
        #[arg(short = 'c', long, value_name = "NAME")]
        component_name: Option<String>,

        /// the title of your component (prompts for value in interactive-mode or error if not provided)
        #[arg(short = 't', long, value_name = "TITLE")]
        component_title: Option<String>,

        /// the version of your component (default: '~' - that is 'none')
        #[arg(short = 'v', long, value_name = "VERSION")]
        component_version: Option<String>,

        /// the title of the site defined in your playbook (prompts for value in interactive-mode or defaults to component's title)
        #[arg(long, value_name = "SITE-TITLE")]
        playbook_site_title: Option<String>,

        /// the key of the template to use for initialization
        #[arg(short = 'k', long, value_name = "TEMPLATE-KEY")]
        init_template_key: Option<String>,

        /// set to add pdf-export to your project (will add the extension to the playbook and create a default config-file)
        #[arg(short = 'p', long, value_name = "FLAG")]
        export_pdf: bool,
    },

    /// builds the site defined in the playbook using antora (currently by running a container of the provided antora_image).
    #[command(about = "builds the site defined in the playbook using antora", long_about = None)]
    Site {
        /// the playbook to build the site from (default: 'antora-playbook.yml')
        #[arg(short = 'p', long, value_name = "PLAYBOOK")]
        playbook: Option<String>,

        /// set to enforce (re-)fetching of remote repositories
        #[arg(short = 'f', long, value_name = "FLAG")]
        fetch: bool,

        /// set to pass-through the stacktrace if antora fails
        #[arg(short = 's', long, value_name = "FLAG")]
        stacktrace: bool,

        /// adjust antora log-level (one of "fatal","error","warn","info","debug","all" or "silent" - default: "warn")
        #[arg(short = 'l', long, value_name = "ANTORALOGLEVEL")]
        log_level: Option<AntoraLogLevel>,

        /// set to open the built site in your default webbrowser (if created successfully)
        #[arg(short = 'o', long, value_name = "FLAG")]
        open: bool,
    },

    /// publishes the site defined in the playbook as confluence page(s) using antora (currently by running a container of the provided antora_image).
    #[command(about = "publishes the site defined in the playbook as confluence page(s)", long_about = None)]
    Confluence {
        /// the confluence-playbook to build the pages from (default: 'confluence-publish.yml')
        #[arg(short = 'p', long, value_name = "PLAYBOOK")]
        playbook: Option<String>,

        /// set to enforce (re-)fetching of remote repositories
        #[arg(short = 'f', long, value_name = "FLAG")]
        fetch: bool,

        /// set to pass-through the stacktrace if antora fails
        #[arg(short = 's', long, value_name = "FLAG")]
        stacktrace: bool,

        /// adjust antora log-level (one of "fatal","error","warn","info","debug","all" or "silent" - default: "warn")
        #[arg(short = 'l', long, value_name = "ANTORALOGLEVEL")]
        log_level: Option<AntoraLogLevel>,
    },
}
