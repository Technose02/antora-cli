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
