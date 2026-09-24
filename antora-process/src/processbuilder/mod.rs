use crate::{Error, Result};
use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread::spawn,
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

    pub fn run<T>(&self) -> Result<(Option<String>, Option<String>)>
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
        cmd.stderr(if self.stdout.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        });

        let mut handle = cmd
            .spawn()
            .map_err(|e| Error::spawn_command_failed(&cmd, e))?;

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
