use antora_fs::{ANTORA_CONFIGURATION, ANTORA_SECRETS_CONFIGURATION};
use antora_process::{
    Error as ProcessError,
    docker::{DockerTool, run_confluence_publish as run_confluence_publish_docker},
    no_op_lineprocessor,
    podman::{
        PodmanTool, run_confluence_publish as run_confluence_publish_podman,
        start_default_machine as podman_start_default_machine,
    },
};
use antora_project::antora_configuration::{
    AntoraConfiguration, ConfluenceConfig, OrphanRemovalStrategy, PlaybookConfig,
};
use relative_path::Filename;

use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use crate::tasks::confluence::pom_xml_template::CONFLUENCE_PAT_ENV_VARIABLE;

use super::site::{
    try_read_antora_configuration, try_read_antora_secrets, validate_playbook_filename,
};
use super::{ConfluenceArgs, get_antora_cache_dir};

mod pom_xml_template;

const XML_FILE: &str = "confluence-publish.xml";

struct TempMavenXml(PathBuf);
impl TempMavenXml {
    pub fn create(project_dir: impl AsRef<Path>, contents: &str) -> Self {
        let mut path = PathBuf::from(project_dir.as_ref());
        if !path.exists() || !path.is_dir() {
            panic!("project-dir must point to an existing dir");
        }
        path.push(XML_FILE);
        fs::write(&path, contents)
            .unwrap_or_else(|_| panic!("writing {XML_FILE} to project-dir must succeed"));
        Self(path)
    }
}

impl Drop for TempMavenXml {
    fn drop(&mut self) {
        _ = fs::remove_file(&self.0);
    }
}

pub fn run(confluence_args: ConfluenceArgs) {
    let mut dir = if let Some(dir) = confluence_args.project_dir {
        PathBuf::from(dir)
    } else {
        PathBuf::from(".")
    };

    // read antora-config-file if possible, fallback to defaults otherwise
    let mut antora_config = match try_read_antora_configuration(&dir) {
        Some(Ok(antora_config)) => antora_config,
        Some(Err(e)) => {
            eprintln!("existing configuration '{ANTORA_CONFIGURATION}' is invalid!: {e}");
            exit(1);
        }
        None => {
            println!("warning: no antora-config found, using defaults");
            AntoraConfiguration {
                playbook: PlaybookConfig::default(),
                antora_image: confluence_args.default_image_config,
                confluence: None,
            }
        }
    };

    // get confluence_config-section or create default and quit
    let confluence_config = if let Some(confluence_config) = &antora_config.confluence {
        confluence_config
    } else {
        eprint!(
            "you have not provided a confluence-section in your {ANTORA_CONFIGURATION}. I created a default one for you which you MUST fill out manually before coming back"
        );
        antora_config.confluence = {
            let confluence_config = ConfluenceConfig {
                root_confluence_url: "<CONFLUENCE_BASEURL>".to_owned(),
                space_key: "WER".to_owned(),
                rest_api_version: "v1".to_owned(),
                page_title_prefix: None,
                page_title_suffix: None,
                skip_ssl_verification: false,
                max_requests_per_second: 10,
                orphan_removal_strategy: OrphanRemovalStrategy::Keep,
                notify_watchers: false,
                projectversion: "unused".to_owned(),
                ..Default::default()
            };

            Some(confluence_config)
        };
        dir.push(ANTORA_CONFIGURATION);
        if let Err(e) = fs::write(&dir, antora_config.to_string()) {
            eprintln!("sorry, could not write to your {ANTORA_CONFIGURATION}: {e}")
        }
        exit(1);
    };

    // read antora-secrets-file if possible, fallback to defaults otherwise
    let antora_secrets = match try_read_antora_secrets(&dir) {
        Some(Ok(antora_config)) => antora_config,
        None => {
            eprintln!(
                "a confluence_pat must be provided but no '{ANTORA_SECRETS_CONFIGURATION}' was found!"
            );
            exit(1);
        }
        _ => {
            eprintln!("existing secrets-file '{ANTORA_SECRETS_CONFIGURATION}' is invalid!");
            exit(1);
        }
    };

    // check for playbook-override via cli-arg
    let playbook_filename = if let Some(playbook_filename) = confluence_args.playbook {
        if let Ok(playbook_filename) = Filename::try_from(playbook_filename.to_owned()) {
            playbook_filename
        } else {
            eprintln!("provided playbook-file is invalid");
            exit(1);
        }
    } else {
        confluence_config.playbook.clone()
    };

    // test if playbook exists and is valid
    let playbook =
        match validate_playbook_filename(&mut dir, &playbook_filename, &confluence_config.playbook)
        {
            Ok(playbook) => playbook,
            Err(msg) => {
                eprintln!("{msg}");
                exit(1);
            }
        };

    // construct container-environment-variables
    let mut env_vars = Vec::with_capacity(3);
    env_vars.push((
        String::from("ANTORA_CACHE_DIR"),
        get_antora_cache_dir(Some(&playbook)),
    ));
    if let Some(git_credentials) = antora_secrets.git_credentials() {
        env_vars.push((String::from("GIT_CREDENTIALS"), git_credentials));
    }
    if let Some(confluence_pat) = antora_secrets.confluence_pat() {
        env_vars.push((String::from(CONFLUENCE_PAT_ENV_VARIABLE), confluence_pat));
    }
    let env_vars = || env_vars.iter().map(|(k, v)| (k.as_str(), v.as_str()));

    let attributes = playbook
        .asciidoc
        .map(|asciidoc_section| asciidoc_section.attributes);

    let temp_maven_xml = TempMavenXml::create(
        &dir,
        pom_xml_template::contents(confluence_config, attributes.as_ref()).as_str(),
    );

    // try with docker first
    let res = {
        let mut docker_cmd = run_confluence_publish_docker(
            &dir,
            env_vars(),
            confluence_args.fetch,
            confluence_args.stacktrace,
            confluence_args.log_level.to_string(),
            playbook_filename.as_ref(),
            &antora_config.antora_image(),
        );
        docker_cmd.with_stdout(no_op_lineprocessor());
        docker_cmd.with_stderr(no_op_lineprocessor());
        let docker_res = docker_cmd.run::<DockerTool>();

        if let Err(ProcessError::CommandNotAvailable(_)) = docker_res {
            println!("docker is not available, trying podman");

            let mut podman_cmd = run_confluence_publish_podman(
                &dir,
                env_vars(),
                confluence_args.fetch,
                confluence_args.stacktrace,
                confluence_args.log_level.to_string(),
                playbook_filename.as_ref(),
                &antora_config.antora_image(),
            );
            podman_cmd.with_stdout(no_op_lineprocessor());
            podman_cmd.with_stderr(no_op_lineprocessor());
            let exec_podman_cmd = || podman_cmd.run::<PodmanTool>();

            let podman_res = exec_podman_cmd();
            if let Err(ProcessError::ProcessExitedWithUnexpectedCode(125)) = podman_res {
                println!("starting podman default machine and retrying..");
                _ = podman_start_default_machine();
                exec_podman_cmd()
            } else {
                podman_res
            }
        } else {
            docker_res
        }
    };

    drop(temp_maven_xml);

    if let Err(e) = res {
        eprintln!("encountered error: {e}");
        exit(1);
    }

    println!(
        "successfully published your confluence pages under ancestor-page {}/spaces/{}/pages/{}",
        confluence_config.root_confluence_url,
        confluence_config.space_key,
        confluence_config.ancestor_id
    );
}
