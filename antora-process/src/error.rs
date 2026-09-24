use core::{error::Error as TError, result::Result as CoreResult};
use std::{fmt::Display, io::Error as IoError, process::Command};

pub type Result<T> = CoreResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    GitNotAvailable,
    UnexpectedOutput,
    GitInitFailed,
    GitAddAllFailed,
    GitCommitFailed,
    ExecDirDoesNotExist,

    CommandNotAvailable(&'static str),
    SpawnCommandFailed(String),
    NoStderrRetrievedFromSpawnedChild,
    NoStdoutRetrievedFromSpawnedChild,
    ProcessExitedWithUnexpectedCode(i32),
    ProcessExitedUnsuccessfully,
    WaitingForProcessFailed(IoError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GitNotAvailable => write!(f, "git is not available on this machine"),
            Error::UnexpectedOutput => write!(f, "received unexpected output from git"),
            Error::GitInitFailed => write!(f, "failed to init git-repo"),
            Error::GitAddAllFailed => write!(f, "failed to stage all changes"),
            Error::GitCommitFailed => write!(f, "failed to commit staged changes"),
            Error::ExecDirDoesNotExist => write!(f, "cannot run process in non-existing dir"),

            Error::CommandNotAvailable(cmd) => write!(f, "CommandNotAvailable: {}", *cmd),
            Error::SpawnCommandFailed(s) => write!(f, "SpawnCommandFailed: '{s}'",),
            Error::NoStderrRetrievedFromSpawnedChild => {
                write!(f, "NoStderrRetrievedFromSpawnedChild")
            }
            Error::NoStdoutRetrievedFromSpawnedChild => {
                write!(f, "NoStdoutRetrievedFromSpawnedChild")
            }
            Error::WaitingForProcessFailed(e) => {
                write!(f, "WaitingForProcessFailed: {e}")
            }
            Error::ProcessExitedWithUnexpectedCode(code) => {
                write!(f, "ProcessExitedWithUnexpectedCode {code}")
            }
            Error::ProcessExitedUnsuccessfully => {
                write!(f, "ProcessExitedUnsuccessfully (no code provided!)")
            }
        }
    }
}

impl Error {
    pub fn spawn_command_failed(cmd: &Command, e: IoError) -> Self {
        Error::SpawnCommandFailed(format!("Spawning '{:#?}' failed: {e:#?}", cmd))
    }
}

impl TError for Error {}
