use crate::{ANTORA_SECRETS_CONFIGURATION, Error, Result};
use antora_project::{antora_playbook::AntoraPlaybook, antora_secrets::AntoraSecrets};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

/// Reads searches and reads the file "antora-secrets.toml" from the provided path.
/// If found, it uses functionality from the antora-project-crate to parse the file into a valid Secrets-Struct.
/// If parsing fails, the file is assumed to be invalid.
pub fn read_antora_secrets_from_dir(dir: impl AsRef<Path>) -> Result<Option<AntoraSecrets>> {
    let mut pb = PathBuf::from(dir.as_ref());
    pb.push(ANTORA_SECRETS_CONFIGURATION);

    if !pb.exists() {
        Ok(None)
    } else {
        let content = read_to_string(&pb).map_err(Error::ReadingFile)?;
        Ok(Some(
            AntoraSecrets::try_from(content.as_str())
                .map_err(|_| Error::FoundSecretsConfigurationIsInvalid)?,
        ))
    }
}

fn yaml_files_iterator(root_dir: &Path) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(str::to_ascii_lowercase)
                .map(|ext| ext == "yml" || ext == "yaml")
                .unwrap_or(false)
        })
}

#[allow(unused)]
pub fn collect_playbooks(root_dir: &Path) -> Vec<AntoraPlaybook> {
    let mut res = Vec::new();

    for yml_file in yaml_files_iterator(root_dir) {
        let yml_file_path = yml_file.path().to_string_lossy();
        println!("trying to serialize yml-file '{yml_file_path}'");
        if let Ok(content) = read_to_string(yml_file.path()) {
            match AntoraPlaybook::try_from(content.as_str()) {
                Ok(playbook) if playbook.content.is_none() => {
                    //println!("is not assumed to be a valid playbook: '{yml_file_path}': has no content-section");
                }
                Ok(playbook) if playbook.site.is_none() => {
                    //println!("is not assumed to be a valid playbook: '{yml_file_path}': has no site-section");
                }
                Ok(playbook) => res.push(playbook),
                Err(_e) => {
                    //eprintln!("{_e}");
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_playbooks_works() {
        let playbooks = collect_playbooks(&PathBuf::from("C:\\tmp\\antora-cli-tests\\test3"));
        println!("found playbooks:");
        println!("{:#?}", playbooks);
    }
}
