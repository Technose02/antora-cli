use super::{CID_FILENAME, ExtTool, ProcessBuilder, Result};
use std::{
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex, OnceLock},
};

static PODMAN_VERSION: OnceLock<Option<String>> = OnceLock::new();

pub struct PodmanTool {}

impl ExtTool for PodmanTool {
    fn available() -> bool {
        Self::podman_version().is_some()
    }
    fn command() -> &'static str {
        "podman"
    }
}

impl PodmanTool {
    pub fn podman_version() -> Option<&'static String> {
        PODMAN_VERSION
            .get_or_init(|| {
                if let Ok(handle) = std::process::Command::new(PodmanTool::command())
                    .arg("--version")
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .stdin(Stdio::null())
                    .spawn()
                    && let Ok(output) = handle.wait_with_output()
                    && output.status.success()
                    && let Ok(version) = str::from_utf8(&output.stdout)
                {
                    let version = {
                        if let Some(stripped) = version.strip_prefix("podman version") {
                            stripped
                        } else {
                            version
                        }
                    }
                    .trim();
                    return Some(String::from(version));
                }

                None
            })
            .into()
    }
}

pub fn start_default_machine(
    container_cleanup_command_target: Arc<Mutex<Option<Command>>>,
) -> Result<()> {
    ProcessBuilder::new()
        .with_args(["machine", "start"])
        .run::<PodmanTool>(container_cleanup_command_target)?;
    Ok(())
}

pub fn run_antora<I, S>(
    dir: &Path,
    env_vars: I,
    fetch: bool,
    stacktrace: bool,
    log_level: String,
    playbook_file: &str,
    image_tag: &str,
) -> ProcessBuilder
where
    I: Iterator<Item = (S, S)>,
    S: AsRef<str>,
{
    let mut pb = ProcessBuilder::new();
    pb.in_dir(dir).with_arg("run");
    for env_var in env_vars {
        pb.with_args([
            "-e",
            format!("{}={}", env_var.0.as_ref(), env_var.1.as_ref()).as_str(),
        ]);
    }
    pb.with_args([
        "--tls-verify=false",
        "--rm",
        "--cidfile",
        CID_FILENAME,
        "-tv",
        ".:/antora:Z",
        image_tag,
    ]);
    if fetch {
        pb.with_arg("--fetch");
    }

    if stacktrace {
        pb.with_arg("--stacktrace");
    }

    pb.with_arg("--log-level").with_arg(log_level);

    pb.with_arg("--attribute").with_arg("building");

    pb.with_arg(playbook_file);
    pb
}

pub fn run_structurizr_export(dir: &Path, image_tag: &str, export_format: &str) -> ProcessBuilder {
    let c4_model_dir = "c4-model";
    let c4_model_path = format!("antora/{c4_model_dir}");
    let exported_views_path = format!("{c4_model_path}/exported_views");
    let workspace_dsl_path = format!("{c4_model_path}/workspace.dsl");
    let workspace_json_file = "workspace.json";
    let workspace_json_path = format!("{c4_model_path}/{workspace_json_file}");

    let workspace_json_locally = {
        let mut p = dir.to_path_buf();
        p.push(c4_model_dir);
        p.push(workspace_json_file);
        p
    };

    let mut pb = ProcessBuilder::new();
    pb.in_dir(dir).with_arg("run");
    pb.with_arg("--rm");
    pb.with_arg("--cidfile").with_arg(CID_FILENAME);
    pb.with_args(["-tv", ".:/antora:Z", image_tag, "structurizr"]);

    pb.with_arg("export");

    if workspace_json_locally.is_file() {
        pb.with_arg("-w").with_arg(workspace_json_path);
    } else {
        println!(
            "warning: '{workspace_json_locally:#?}' not found; falling back to DSL, so explicit layout-information will not be considered!"
        );
        pb.with_arg("-w").with_arg(workspace_dsl_path);
    }

    pb.with_arg("-f").with_arg(export_format);
    pb.with_arg("-o").with_arg(exported_views_path);

    pb
}

pub fn run_structurizr_local(dir: &Path, image_tag: &str, host_port: u16) -> ProcessBuilder {
    let mut pb = ProcessBuilder::new();

    pb.in_dir(dir)
        .with_arg("run")
        .with_arg("--rm")
        .with_arg("--cidfile")
        .with_arg(CID_FILENAME)
        .with_arg("-t");
    pb.with_arg("-p").with_arg(format!("{}:8080", host_port));
    pb.with_arg("-v").with_arg(".:/antora:Z");
    pb.with_arg(image_tag);
    pb.with_arg("structurizr")
        .with_arg("local")
        .with_arg("/antora/c4-model");

    pb
}

pub fn run_confluence_publish<I, S>(
    dir: &Path,
    env_vars: I,
    fetch: bool,
    stacktrace: bool,
    log_level: String,
    playbook_file: &str,
    image_tag: &str,
) -> ProcessBuilder
where
    I: Iterator<Item = (S, S)>,
    S: AsRef<str>,
{
    let mut pb = ProcessBuilder::new();
    pb.in_dir(dir).with_arg("run");
    for env_var in env_vars {
        pb.with_args([
            "-e",
            format!("{}={}", env_var.0.as_ref(), env_var.1.as_ref()).as_str(),
        ]);
    }
    pb.with_args([
        "--tls-verify=false",
        "--rm",
        "--cidfile",
        CID_FILENAME,
        "-tv",
        ".:/antora:Z",
        image_tag,
        "confluence-publish",
    ]);
    if fetch {
        pb.with_arg("--fetch");
    }

    if stacktrace {
        pb.with_arg("--stacktrace");
    }

    pb.with_arg("--log-level").with_arg(log_level);

    pb.with_arg("--attribute").with_arg("building");

    pb.with_arg("--attribute")
        .with_arg("confluence-publish=true");

    pb.with_arg("--ui-bundle-url")
        .with_arg("/ui/default/ui-bundle.zip");

    pb.with_arg("--extension")
        .with_arg("/custom-extensions/vanilladoc-lite.js");

    pb.with_arg(playbook_file);
    pb
}
