use crate::tasks::{ExportFormat, site::try_read_antora_configuration};
use antora_fs::ANTORA_CONFIGURATION;
use antora_process::{
    Error as ProcessError,
    docker::{
        DockerTool, run_structurizr_export as run_structurizr_export_docker,
        run_structurizr_local as run_structurizr_local_docker,
    },
    no_op_lineprocessor,
    podman::{
        PodmanTool, run_structurizr_export as run_structurizr_export_podman,
        start_default_machine as podman_start_default_machine,
    },
};

use antora_project::antora_configuration::{AntoraConfiguration, ImageConfig, PlaybookConfig};
use std::{path::PathBuf, process::exit};

const DEFAULT_STRUCTURIZR_HOST_PORT: u16 = 8080;
pub const STRUCTURIZR_CONTAINER_PORT: u16 = 8080;

pub fn run_export_diagrams(
    project_dir: Option<&String>,
    default_image_config: ImageConfig,
    export_format: Option<&ExportFormat>,
) {
    let dir = if let Some(dir) = project_dir {
        PathBuf::from(dir)
    } else {
        PathBuf::from(".")
    };

    if !dir.is_dir() {
        eprintln!(
            "error: directory '{}' does not exist!",
            dir.to_string_lossy()
        );
        exit(1);
    }

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
                antora_image: default_image_config,
                confluence: None,
            }
        }
    };

    let export_format = if let Some(export_format) = export_format {
        export_format.as_ref().to_string()
    } else {
        ExportFormat::default().as_ref().to_string()
    };

    let res = {
        let mut docker_cmd = run_structurizr_export_docker(
            &dir,
            &antora_config.antora_image(),
            export_format.as_str(),
        );
        docker_cmd.with_stdout(no_op_lineprocessor());
        docker_cmd.with_stderr(no_op_lineprocessor());
        let docker_res = docker_cmd.run::<DockerTool>();

        if let Err(ProcessError::CommandNotAvailable(_)) = docker_res {
            println!("docker is not available, trying podman");

            let mut podman_cmd = run_structurizr_export_podman(
                &dir,
                &antora_config.antora_image(),
                export_format.as_str(),
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

    if let Err(e) = res {
        eprintln!("encountered error: {e}");
        exit(1);
    }

    println!("diagrams successfully exported");
}

pub fn run_local(
    project_dir: Option<&String>,
    default_image_config: ImageConfig,
    selected_host_port: Option<u16>,
) {
    let dir = if let Some(dir) = project_dir {
        PathBuf::from(dir)
    } else {
        PathBuf::from(".")
    };

    if !dir.is_dir() {
        eprintln!(
            "error: directory '{}' does not exist!",
            dir.to_string_lossy()
        );
        exit(1);
    }

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
                antora_image: default_image_config,
                confluence: None,
            }
        }
    };

    let host_port = if let Some(port) = selected_host_port {
        port
    } else {
        DEFAULT_STRUCTURIZR_HOST_PORT
    };

    let res = {
        let mut docker_cmd =
            run_structurizr_local_docker(&dir, &antora_config.antora_image(), host_port);
        docker_cmd.with_stdout(no_op_lineprocessor());
        docker_cmd.with_stderr(no_op_lineprocessor());
        let docker_res = docker_cmd.run::<DockerTool>();

        if let Err(ProcessError::CommandNotAvailable(_)) = docker_res {
            println!("docker is not available, trying podman");

            let mut podman_cmd =
                run_structurizr_local_docker(&dir, &antora_config.antora_image(), host_port);
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

    if let Err(e) = res {
        eprintln!("encountered error: {e}");
        exit(1);
    }

    println!("diagrams successfully exported");
}
