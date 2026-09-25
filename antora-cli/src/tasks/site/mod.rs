use antora_fs::{ANTORA_CONFIGURATION, ANTORA_SECRETS_CONFIGURATION, NormalizePath};
use antora_process::{
    Error as ProcessError, LineProcessor,
    docker::{DockerTool, run_antora as run_antora_docker},
    no_op_lineprocessor,
    podman::{
        PodmanTool, run_antora as run_antora_podman,
        start_default_machine as podman_start_default_machine,
    },
};
use antora_project::{
    Result as ProjectResult,
    antora_configuration::{AntoraConfiguration, PlaybookConfig},
    antora_playbook::AntoraPlaybook,
    antora_secrets::AntoraSecrets,
};
use regex::Regex;
use relative_path::Filename;

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    process::exit,
    sync::LazyLock,
};

use super::{SiteArgs, get_antora_cache_dir};

static OPEN_SITE_EXTRACTOR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Open file:\/\/(.*) in a browser to view your site\.")
        .expect("static pattern for open-site must compile")
});

pub fn try_read_antora_configuration(dir: &Path) -> Option<ProjectResult<AntoraConfiguration>> {
    let mut p = dir.to_owned();
    p.push(ANTORA_CONFIGURATION);
    if p.exists() && p.is_file() {
        if let Ok(contents) = read_to_string(p) {
            Some(AntoraConfiguration::try_from(contents.as_str()))
        } else {
            eprintln!("found invalid {ANTORA_CONFIGURATION} file, aborting");
            exit(1);
        }
    } else {
        None
    }
}

pub fn try_read_antora_secrets(dir: &Path) -> Option<ProjectResult<AntoraSecrets>> {
    let mut p = dir.to_owned();
    p.push(ANTORA_SECRETS_CONFIGURATION);
    if p.exists() && p.is_file() {
        if let Ok(contents) = read_to_string(p) {
            Some(AntoraSecrets::try_from(contents.as_str()))
        } else {
            eprintln!("found invalid {ANTORA_SECRETS_CONFIGURATION} file, aborting");
            exit(1);
        }
    } else {
        None
    }
}

pub fn validate_playbook_filename(
    project_dir: &mut PathBuf,
    provided_playbook_filename: &Filename,
    configured_playbook_filename: &Filename,
) -> Result<AntoraPlaybook, String> {
    project_dir.push(Into::<PathBuf>::into(provided_playbook_filename.clone()));
    if !project_dir.exists() || !project_dir.is_file() {
        project_dir.pop();

        Err(format!(
            "error running task: playbook-file '{}' not found",
            configured_playbook_filename.clone()
        ))
    } else if let Ok(contents) = read_to_string(&project_dir) {
        project_dir.pop();

        AntoraPlaybook::try_from(contents.as_str()).map_err(|e| {
            format!(
                "error running task: file '{}' is not a valid antora-playbook ({e})",
                configured_playbook_filename
            )
        })
    } else {
        project_dir.pop();

        Err(format!(
            "error running task: file '{}' could not be read",
            configured_playbook_filename
        ))
    }
}

pub fn run(site_args: SiteArgs) {
    let mut dir = if let Some(dir) = site_args.project_dir {
        PathBuf::from(dir)
    } else {
        PathBuf::from(".")
    };

    // read antora-config-file if possible, fallback to defaults otherwise
    let antora_config = match try_read_antora_configuration(&dir) {
        Some(Ok(antora_config)) => antora_config,
        Some(Err(e)) => {
            eprintln!("existing configuration '{ANTORA_CONFIGURATION}' is invalid!: {e}");
            exit(1);
        }
        None => {
            println!("warning: no antora-config found, using defaults");
            AntoraConfiguration {
                playbook: PlaybookConfig::default(),
                antora_image: site_args.default_image_config,
                confluence: None,
            }
        }
    };

    // check for playbook-override via cli-arg
    let playbook_filename = if let Some(playbook_filename) = site_args.playbook_filename {
        if let Ok(playbook_filename) = Filename::try_from(playbook_filename.to_owned()) {
            playbook_filename
        } else {
            eprintln!("provided playbook-file is invalid");
            exit(1);
        }
    } else {
        antora_config.playbook_filename().clone()
    };

    // test if playbook exists and is valid
    let playbook = match validate_playbook_filename(
        &mut dir,
        &playbook_filename,
        antora_config.playbook_filename(),
    ) {
        Ok(playbook) => playbook,
        Err(msg) => {
            eprintln!("{msg}");
            exit(1);
        }
    };

    // prepare line-processor for stdout
    let normalized_path = if let Ok(p) = dir.try_normalize() {
        p
    } else {
        dir.to_string_lossy().into_owned()
    };
    let build_site_url_fixer = LineProcessor::new(move |line| {
        line.replace(
            "Open file:///antora/",
            &format!("Open file://{normalized_path}/"),
        )
    });

    // read antora-secrets-file if possible, fallback to defaults otherwise
    let antora_secrets = match try_read_antora_secrets(&dir) {
        Some(Ok(antora_config)) => antora_config,
        None => {
            println!(
                "warning: no '{ANTORA_SECRETS_CONFIGURATION}' found, fetching remote repositories might fail"
            );
            AntoraSecrets::default()
        }
        _ => {
            eprintln!("existing secrets-file '{ANTORA_SECRETS_CONFIGURATION}' is invalid!");
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
    let env_vars = || env_vars.iter().map(|(k, v)| (k.as_str(), v.as_str()));

    // try with docker first
    let res = {
        let mut docker_cmd = run_antora_docker(
            &dir,
            env_vars(),
            site_args.fetch,
            site_args.stacktrace,
            site_args.log_level.to_string(),
            playbook_filename.as_ref(),
            &antora_config.antora_image(),
        );
        docker_cmd.with_stdout(build_site_url_fixer.clone());
        docker_cmd.with_stderr(no_op_lineprocessor());
        let docker_res = docker_cmd.run::<DockerTool>();

        if let Err(ProcessError::CommandNotAvailable(_)) = docker_res {
            println!("docker is not available, trying podman");

            let mut podman_cmd = run_antora_podman(
                &dir,
                env_vars(),
                site_args.fetch,
                site_args.stacktrace,
                site_args.log_level.to_string(),
                playbook_filename.as_ref(),
                &antora_config.antora_image(),
            );
            podman_cmd.with_stdout(build_site_url_fixer);
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
    match res {
        Ok((Some(stdout), _)) if site_args.open => {
            if let Some(captures) = OPEN_SITE_EXTRACTOR_REGEX.captures(&stdout) {
                let mut built_site_url = format!("file:///{}", &captures[1]);
                built_site_url = built_site_url.replace("\\", "/");

                if let Err(e) = webbrowser::open(&built_site_url) {
                    eprintln!(
                        "failed to open url of built site '{built_site_url}' in default browser: {e}"
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("encountered error: {e}");
            exit(1);
        }
        _ => {}
    }
}
