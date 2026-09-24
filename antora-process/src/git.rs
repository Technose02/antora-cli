use crate::{Error, Result};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const GIT_COMMAND: &str = "git";
const ON_BRANCH_PREFIX: &str = "On branch ";

pub enum RepoConstellationForPath {
    NoRepo,
    RepoAndRoot(String),
    RepoButNotRoot(String),
}

impl RepoConstellationForPath {
    pub fn check(path: &Option<impl AsRef<Path>>) -> Result<Self> {
        if let Some(path) = path
            && !path.as_ref().exists()
        {
            // parent could still be inside a git-repos and creating the path would result in RepoButNotRoot
            let mut path = path.as_ref();
            while let Some(p) = path.parent() {
                path = p;
                if p.exists()
                    && let Some(branch) = path_is_git_repo(&Some(p))?
                {
                    return Ok(RepoConstellationForPath::RepoButNotRoot(branch));
                }
            }
            return Ok(RepoConstellationForPath::NoRepo);
        }
        if let Some(branch) = path_is_git_repo(path)? {
            if path_has_dotgit_dir(path)? {
                Ok(RepoConstellationForPath::RepoAndRoot(branch))
            } else {
                Ok(RepoConstellationForPath::RepoButNotRoot(branch))
            }
        } else {
            Ok(RepoConstellationForPath::NoRepo)
        }
    }
}

fn try_git_command_in_path(path: &Option<impl AsRef<Path>>) -> Result<Command> {
    let mut cmd = Command::new(GIT_COMMAND);
    if let Some(p) = path {
        if p.as_ref().exists() && p.as_ref().is_dir() {
            cmd.current_dir(p);
        } else {
            return Err(Error::ExecDirDoesNotExist);
        }
    } else {
        cmd.current_dir(".");
    }
    Ok(cmd)
}

fn git_command_in_path(path: &Option<impl AsRef<Path>>) -> Command {
    try_git_command_in_path(path).expect(
        "path-argument to git_command_in_path is always expected to exist and be a valid dir",
    )
}

pub fn git_available() -> bool {
    if let Ok(output) = Command::new(GIT_COMMAND)
        .current_dir(".")
        .arg("--version")
        .stdout(Stdio::piped())
        .output()
    {
        output.stdout.starts_with("git version ".as_bytes())
    } else {
        false
    }
}

pub fn path_is_git_repo(path: &Option<impl AsRef<Path>>) -> Result<Option<String>> {
    if git_available() {
        if let Ok(output) = git_command_in_path(path)
            .arg("status")
            .stdout(Stdio::piped())
            .output()
        {
            if !output.stdout.starts_with(ON_BRANCH_PREFIX.as_bytes()) {
                Ok(None)
            } else {
                let mut output =
                    String::try_from(output.stdout).map_err(|_| Error::UnexpectedOutput)?;
                output = output
                    .split("\n")
                    .next()
                    .ok_or(Error::UnexpectedOutput)?
                    .strip_prefix(ON_BRANCH_PREFIX)
                    .unwrap()
                    .trim()
                    .to_string();
                Ok(Some(output))
            }
        } else {
            Err(Error::UnexpectedOutput)
        }
    } else {
        Err(Error::GitNotAvailable)
    }
}

pub fn path_has_dotgit_dir(path: &Option<impl AsRef<Path>>) -> Result<bool> {
    if git_available() {
        let mut pb = if let Some(p) = path {
            p.as_ref().into()
        } else {
            PathBuf::from(".")
        };
        pb.push(".git");
        Ok(pb.exists() && pb.is_dir())
    } else {
        Err(Error::GitNotAvailable)
    }
}

pub fn init_in_dir(path: &Option<impl AsRef<Path>>) -> Result<()> {
    if git_available() {
        if let Ok(output) = git_command_in_path(path)
            .arg("init")
            .stdout(Stdio::null())
            .output()
        {
            if !output.status.success() {
                Err(Error::GitInitFailed)
            } else {
                Ok(())
            }
        } else {
            Err(Error::GitInitFailed)
        }
    } else {
        Err(Error::GitNotAvailable)
    }
}

pub fn add_all_and_commit(path: &Option<impl AsRef<Path>>, message: &str) -> Result<()> {
    if git_available() {
        if let Ok(output) = git_command_in_path(path)
            .arg("add")
            .arg("-A")
            .stdout(Stdio::null())
            .output()
            && output.status.success()
        {
            if let Ok(output) = git_command_in_path(path)
                .arg("commit")
                .arg("-m")
                .arg(message)
                .stdout(Stdio::null())
                .output()
            {
                if output.status.success() {
                    Ok(())
                } else {
                    Err(Error::GitCommitFailed)
                }
            } else {
                Err(Error::GitCommitFailed)
            }
        } else {
            Err(Error::GitAddAllFailed)
        }
    } else {
        Err(Error::GitNotAvailable)
    }
}

pub fn init_and_commit_adding_all(path: &Option<impl AsRef<Path>>, message: &str) -> Result<()> {
    init_in_dir(path)?;
    add_all_and_commit(path, message)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn git_available_works() {
        assert!(git_available())
    }

    #[test]
    fn path_is_git_repo_works() {
        assert!(matches!(path_is_git_repo(&None::<&str>), Ok(Some(_))));
        assert!(matches!(path_is_git_repo(&Some("c:\\")), Ok(None)));
    }

    #[test]
    fn path_has_dotgit_dir_works() {
        // we are in antora-process, the repo is in the workspace ;-)
        assert!(matches!(path_has_dotgit_dir(&None::<&str>), Ok(false)));
        assert!(matches!(path_has_dotgit_dir(&Some("../")), Ok(true)));
        assert!(matches!(path_has_dotgit_dir(&Some("c:\\")), Ok(false)));
    }

    #[test]
    fn repo_constellation_for_path_works() {
        assert!(matches!(
            RepoConstellationForPath::check(&None::<&str>),
            Ok(RepoConstellationForPath::RepoButNotRoot(_))
        ));
        assert!(matches!(
            RepoConstellationForPath::check(&Some("../")),
            Ok(RepoConstellationForPath::RepoAndRoot(_))
        ));
        assert!(matches!(
            RepoConstellationForPath::check(&Some("c:\\")),
            Ok(RepoConstellationForPath::NoRepo)
        ));
    }
}
