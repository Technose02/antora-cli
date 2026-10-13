use crate::{CID_FILENAME, Error, Result};
use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread::spawn,
    time::{Duration, Instant},
};

mod lineprocessor;
pub use lineprocessor::{LineProcessor, no_op_lineprocessor};

pub trait ExtTool {
    fn command() -> &'static str;
    fn available() -> bool;
}

#[derive(Default, Debug)]
pub struct ProcessBuilder {
    dir: Option<PathBuf>,
    args: Vec<String>,
    stdout: Option<LineProcessor>,
    stderr: Option<LineProcessor>,
}

impl ProcessBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn in_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.dir = Some(dir.as_ref().into());
        self
    }

    pub fn with_arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn with_args<A, I>(&mut self, args: A) -> &mut Self
    where
        A: IntoIterator<Item = I>,
        I: Into<String>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    pub fn with_stdout(&mut self, stdout_line_processor: LineProcessor) -> &mut Self {
        self.stdout = Some(stdout_line_processor);
        self
    }
    pub fn with_stderr(&mut self, stderr_line_processor: LineProcessor) -> &mut Self {
        self.stderr = Some(stderr_line_processor);
        self
    }

    pub fn run<T>(
        &self,
        container_cleanup_command_target: Arc<Mutex<Option<Command>>>,
    ) -> Result<(Option<String>, Option<String>)>
    where
        T: ExtTool,
    {
        if !T::available() {
            return Err(Error::CommandNotAvailable(T::command()));
        }
        let mut cmd = Command::new(T::command());
        if let Some(dir) = &self.dir {
            cmd.current_dir(dir);
        }

        for arg in &self.args {
            cmd.arg(arg);
        }

        cmd.stdin(Stdio::null());
        cmd.stdout(if self.stdout.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        });
        cmd.stderr(if self.stderr.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        });

        let mut handle = cmd
            .spawn()
            .map_err(|e| Error::spawn_command_failed(&cmd, e))?;

        let cmd_dir = cmd.get_current_dir().map(|p| p.to_owned());
        std::thread::spawn(move || {
            let cid_path = {
                let mut path = if let Some(path) = cmd_dir {
                    path
                } else {
                    if let Ok(path) = std::env::current_dir() {
                        path
                    } else {
                        "./".into()
                    }
                };
                path.push(CID_FILENAME);
                path
            };

            let started = std::time::Instant::now();
            while Instant::now().duration_since(started) <= Duration::from_millis(5000) {
                if cid_path.is_file()
                    && let Ok(cid) = std::fs::read_to_string(&cid_path)
                    && !cid.is_empty()
                {
                    let mut cleanup_command = Command::new(cmd.get_program());
                    cleanup_command.arg("container").arg("kill").arg(&cid);

                    let mut lock = container_cleanup_command_target.lock();
                    let container_cleanup_command_target = lock
                        .as_deref_mut()
                        .expect("failed to lock on container_cleanup_map");
                    *container_cleanup_command_target = Some(cleanup_command);
                    println!("registered container_cleanup_command_target with cid '{cid}'");
                    std::thread::spawn(move || {
                        let started = std::time::Instant::now();
                        while Instant::now().duration_since(started) <= Duration::from_millis(5000)
                        {
                            if std::fs::remove_file(&cid_path).is_ok() {
                                return;
                            }
                            std::thread::sleep(Duration::from_millis(10));
                        }
                        eprintln!("failed to remove cidfile '{cid_path:#?}'");
                    });
                    return;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            println!("no cid detected");
        });

        let stdout_handle = if let Some(stdout_line_processor) = self.stdout.clone() {
            let stdout_lines_iter = BufReader::new(
                handle
                    .stdout
                    .take()
                    .ok_or(Error::NoStdoutRetrievedFromSpawnedChild)?,
            )
            .lines();

            let stdout_prefix = format!("{} [OUT]", T::command());
            let handle = spawn(move || {
                let mut lines = Vec::new();
                for line_res in stdout_lines_iter {
                    if let Ok(line) = line_res {
                        let processed = stdout_line_processor.run(line);
                        println!("{stdout_prefix}: {processed}");
                        lines.push(processed);
                    } else {
                        panic!("processbuilder::110");
                        //break;
                    }
                }
                lines.join("\n")
            });
            Some(handle)
        } else {
            None
        };

        let stderr_handle = if let Some(stderr_line_processor) = self.stderr.clone() {
            let stderr_lines_iter = BufReader::new(
                handle
                    .stderr
                    .take()
                    .ok_or(Error::NoStderrRetrievedFromSpawnedChild)?,
            )
            .lines();

            let stderr_prefix = format!("{} [ERR]", T::command());
            let handle = spawn(move || {
                let mut lines = Vec::new();
                for line_res in stderr_lines_iter {
                    if let Ok(line) = line_res {
                        let processed = stderr_line_processor.run(line);
                        println!("{stderr_prefix}: {processed}");
                        lines.push(processed);
                    } else {
                        panic!("processbuilder::134");
                        //break;
                    }
                }
                lines.join("\n")
            });
            Some(handle)
        } else {
            None
        };

        let es = handle.wait().map_err(Error::WaitingForProcessFailed)?;
        let std_out: Option<String> = stdout_handle.and_then(|h| h.join().ok());
        let std_err: Option<String> = stderr_handle.and_then(|h| h.join().ok());

        if es.success() {
            Ok((std_out, std_err))
        } else if let Some(code) = es.code() {
            Err(Error::ProcessExitedWithUnexpectedCode(code))
        } else {
            Err(Error::ProcessExitedUnsuccessfully)
        }
    }
}
