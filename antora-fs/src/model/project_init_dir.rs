use crate::{
    Error,
    check::{
        check_dir_contains_antora_configuration, check_dir_contains_default_playbook,
        check_dir_is_empty_or_does_not_exist,
    },
};
use std::path::{Path, PathBuf};

/// Represents a valid directory for initializing an antora-repo
pub struct ProjectInitDir(PathBuf);

impl TryFrom<&str> for ProjectInitDir {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut pb = PathBuf::from(".");
        if !value.is_empty() {
            pb.push(value);
        };

        if check_dir_is_empty_or_does_not_exist(&pb) {
            return Ok(ProjectInitDir(pb));
        }

        if check_dir_contains_antora_configuration(&pb) {
            Err(Error::InitDirContainsAntoraConfiguration)
        } else if check_dir_contains_default_playbook(&pb) {
            Err(Error::InitDirContainsDefaultPlaybook)
        } else {
            Ok(ProjectInitDir(pb))
        }
    }
}

impl AsRef<Path> for ProjectInitDir {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl From<ProjectInitDir> for PathBuf {
    fn from(value: ProjectInitDir) -> Self {
        value.0
    }
}

impl ProjectInitDir {
    pub fn exists(&self) -> bool {
        self.0.exists() && self.0.is_dir()
    }
}
