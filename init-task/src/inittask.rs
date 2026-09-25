use antora_fs::{Error as AntoraFsError, ProjectInitDir, Vfs, read_antora_secrets_from_dir};
use antora_process::{
    Error as ProcessError,
    git::{RepoConstellationForPath, init_and_commit_adding_all},
};
use antora_project::{antora_secrets::AntoraSecrets, component_version::ComponentVersion};
use relative_path::RelativeFile;
use std::{path::Path, process::exit};

use super::InitArgs;
use crate::{TemplateResolver, assistant::InitAssistant};

pub struct InitTask;

impl InitTask {
    fn validate_project_init_dir_or_exit(project_init_dir: Option<&String>) -> ProjectInitDir {
        let project_init_dir = project_init_dir
            .as_ref()
            .map(|string_ref| string_ref.as_str())
            .unwrap_or("");
        match ProjectInitDir::try_from(project_init_dir) {
            Ok(project_init_dir) => project_init_dir,
            Err(AntoraFsError::InitDirContainsAntoraConfiguration) => {
                eprintln!(
                    "the provided project-dir contains a antora-configuration and cannot be used"
                );
                std::process::exit(1);
            }
            Err(AntoraFsError::InitDirContainsDefaultPlaybook) => {
                eprintln!(
                    "the provided project-dir contains a default playbook and cannot be used."
                );
                exit(1);
            }
            Err(e) => {
                eprintln!("unexpected error: {e}\nquitting");
                exit(1);
            }
        }
    }

    fn check_or_create_secrets_exit_if_encountered_invalid(
        project_init_dir: &ProjectInitDir,
        vfs: &Vfs,
    ) {
        match read_antora_secrets_from_dir(project_init_dir) {
            Ok(None) => {
                vfs.write_antora_secrets_configuration(AntoraSecrets::default().to_string());
            }
            Err(AntoraFsError::FoundSecretsConfigurationIsInvalid) => {
                eprintln!(
                    "the provided project-dir contains an invalid antora-secrets-configuration and cannot be used"
                );
                exit(1);
            }
            _ => {}
        };
    }

    fn check_git_constellation_exit_if_nested(project_init_dir: &ProjectInitDir) -> bool {
        // in order to be valid for antora-repo initialization the project_init_dir must either
        // - not be a git repo or
        // - be a non-nested git repo - that is: it contains a .git-dir ;-)

        match RepoConstellationForPath::check(&Some(project_init_dir.as_ref())) {
            Ok(RepoConstellationForPath::NoRepo) => true, // ok, need to init new git repo,
            Ok(RepoConstellationForPath::RepoAndRoot(branch))
                if branch == "main" || branch == "master" =>
            {
                println!(
                    "you are initializing into an existing git-repository on branch '{branch}' - will continue but not stage/commit any changes"
                );
                false
            } // existing repo compatible,
            Ok(RepoConstellationForPath::RepoAndRoot(branch)) => {
                println!(
                    "you are initializing into a git-repository at branch '{branch}'; the playbook however will be created to include a local content-root on branch 'main' or 'master'."
                );
                false
            }
            Ok(RepoConstellationForPath::RepoButNotRoot(_)) => {
                eprintln!(
                    "you are initializing into a git-repository but not at the root-level. This would lead to problems and is currently not supported"
                );
                exit(1);
            }
            Err(ProcessError::GitNotAvailable) => {
                eprintln!("git is needed but not available on your machine");
                exit(1);
            }
            Err(ProcessError::UnexpectedOutput) => {
                eprintln!("error running git on your machine (unexpected output)");
                exit(1);
            }
            Err(_) => unreachable!("other error-variants not relevant for this context"),
        }
    }

    fn check_for_conflicts_persisting_or_exitting(vfs: &Vfs) -> u64 {
        let conflicts = vfs.check_for_conflicts();
        if conflicts.is_empty() {
            match vfs.persist() {
                Ok(number_written) => number_written,
                Err(e) => {
                    eprintln!("encountered an error while persisting: {e}");
                    exit(1);
                }
            }
        } else {
            eprintln!(
                "aborting due to conflicts ; since the following files would be overwritten:"
            );
            for pb in conflicts.iter() {
                eprintln!("\t{}", pb.to_string_lossy());
            }
            exit(1);
        }
    }

    fn create_git_repo_or_exit(path: &Path, message: &str) {
        match init_and_commit_adding_all(&Some(path), message) {
            Err(ProcessError::GitInitFailed) => {
                eprintln!("failed to init git repo");
                exit(1);
            }
            Err(ProcessError::GitAddAllFailed) | Err(ProcessError::GitCommitFailed) => {
                eprintln!("failed to commit changes to git repo");
                exit(1);
            }
            Err(e) => {
                eprintln!(
                    "encountered an error while initializing git repo and committing changes: {e}"
                );
                exit(1);
            }
            _ => {}
        }
    }

    pub fn run(init_args: InitArgs, template_resolver: Box<dyn TemplateResolver>) {
        // create verified project_dir for repo-initialization
        let project_init_dir = Self::validate_project_init_dir_or_exit(init_args.project_dir);

        // check for invalid git-constellation
        let create_git_repo = Self::check_git_constellation_exit_if_nested(&project_init_dir);

        // create a virtual FS for preparing all files that need to be created.
        let vfs = Vfs::from(&project_init_dir);

        // check for a secrets-configuration. If not found we create a default and write it to the vfs.
        Self::check_or_create_secrets_exit_if_encountered_invalid(&project_init_dir, &vfs);

        let results = {
            let mut assistant = InitAssistant::new(!init_args.non_interactive_flag);
            assistant
                .with_provided_content_source_root(init_args.provided_docs_dir)
                .with_provided_component_name(init_args.provided_component_name)
                .with_provided_component_title(init_args.provided_component_title)
                .with_provided_playbook_site_title(init_args.provided_playbook_site_title)
                .with_provided_init_template_key(init_args.provided_init_template_key);

            assistant.process_exitting_eventually(&*template_resolver)
        };

        // create template according to args
        let component_version = if let Some(component_version) =
            init_args.provided_component_version
        {
            if let Ok(component_version) = ComponentVersion::try_from(component_version.clone()) {
                component_version
            } else {
                eprintln!("invalid component-version provided. Aborting");
                exit(1);
            }
        } else {
            ComponentVersion::empty()
        };

        let mut init_template = template_resolver
            .try_resolve(&results, &component_version, init_args.include_scaffolding)
            .expect("Template must resolve from valid InitAssistantResults");

        // create/update files according to template
        init_template.process(&vfs, init_args.export_pdf);

        // write .gitignore using content created from template
        vfs.write_gitignore(init_template.get_gitignore_content());

        // get updated antora_configuration from template
        let antora_configuration = init_template.get_antora_configuration(&*template_resolver);

        // optionally refine antora_configuration further

        // write antora_configuration
        vfs.write_antora_configuration(antora_configuration.to_string());

        // get component_version_descriptor if template defines one
        if let Some(component_version_descriptor) = init_template.get_component_version_descriptor()
        {
            // optionally refine component_version_descriptor further

            vfs.write_component_version_descriptor(
                results.content_source_root(),
                component_version_descriptor,
            );
        }

        // write antora_playbook with filename as defined in antora_configuration
        if let Some(antora_playbook) = init_template.get_antora_playbook() {
            // optionally refine antora_playbook further

            vfs.write_antora_playbook_as(
                RelativeFile::from(antora_configuration.playbook_filename().clone()),
                antora_playbook.to_string(),
            );
        }

        // check for any conflicts with existing filesystem and persist if ok or cancel
        Self::check_for_conflicts_persisting_or_exitting(&vfs);

        // create a git-repo if ok according to previous git-analysis
        if create_git_repo {
            Self::create_git_repo_or_exit(project_init_dir.as_ref(), "initial commit");
        }
    }
}
