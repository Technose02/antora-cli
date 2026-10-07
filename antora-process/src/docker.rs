use super::{ExtTool, ProcessBuilder};
use std::{path::Path, process::Stdio, sync::OnceLock};

static DOCKER_VERSION: OnceLock<Option<String>> = OnceLock::new();

pub struct DockerTool {}

impl ExtTool for DockerTool {
    fn available() -> bool {
        Self::docker_version().is_some()
    }
    fn command() -> &'static str {
        "docker"
    }
}

impl DockerTool {
    pub fn docker_version() -> Option<&'static String> {
        DOCKER_VERSION
            .get_or_init(|| {
                if let Ok(handle) = std::process::Command::new(DockerTool::command())
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
                        if let Some(stripped) = version.strip_prefix("Docker version") {
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
            format!("{}=\"{}\"", env_var.0.as_ref(), env_var.1.as_ref()).as_str(),
        ]);
    }
    pb.with_args(["--rm", "-tv", ".:/antora:Z", image_tag]);

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
    let mut pb = ProcessBuilder::new();
    pb.in_dir(dir).with_arg("run");
    pb.with_args(["--rm", "-tv", ".:/antora:Z", image_tag, "structurizr"]);

    pb.with_arg("export");
    pb.with_arg("-w").with_arg("/antora/c4-model/workspace.dsl");
    pb.with_arg("-f").with_arg(export_format);
    pb.with_arg("-o")
        .with_arg("/antora/c4-model/exported_diagrams");

    pb
}

pub fn run_structurizr_local(dir: &Path, image_tag: &str, host_port: u16) -> ProcessBuilder {
    let mut pb = ProcessBuilder::new();
    pb.in_dir(dir)
        .with_arg("run")
        .with_arg("--rm")
        .with_arg("-t");
    pb.with_arg("-p").with_arg(format!("{}:8080", host_port));
    pb.with_arg("-v").with_arg(".:/antora:Z");
    pb.with_arg(image_tag);
    pb.with_arg("structurizr").with_arg("local");

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
        "--rm",
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
