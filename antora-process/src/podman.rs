use std::{path::Path, process::Stdio, sync::OnceLock};

use super::{ExtTool, ProcessBuilder, Result};

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

pub fn start_default_machine() -> Result<()> {
    ProcessBuilder::new()
        .with_args(["machine", "start"])
        .run::<PodmanTool>()?;
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
