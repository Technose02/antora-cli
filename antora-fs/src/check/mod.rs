use crate::ANTORA_CONFIGURATION;
use antora_project::DEFAULT_PLAYBOOK_FILENAME;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn check_dir_is_empty_or_does_not_exist(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    if path.exists() {
        if !path.is_dir() {
            // the provided path exists but is no directory
            false
        } else {
            // path exists and is a directory: check if it is empty
            match read_dir(path) {
                Err(_) => false, // could not be read, seems to be a problem with this path
                Ok(read_dir) => read_dir.count() == 0, // entry-count == 0 -> empty dir
            }
        }
    } else {
        // path does not exist
        true
    }
}

fn check_wellknown_file_exists_in_path(path: impl Into<PathBuf>, filename: &str) -> bool {
    let mut path = path.into();
    path.push(filename);
    path.exists()
}

pub fn check_dir_contains_antora_configuration(path: impl Into<PathBuf>) -> bool {
    check_wellknown_file_exists_in_path(path, ANTORA_CONFIGURATION)
}

pub fn check_dir_contains_default_playbook(path: impl Into<PathBuf>) -> bool {
    check_wellknown_file_exists_in_path(path, DEFAULT_PLAYBOOK_FILENAME)
}
