use crate::{
    ANTORA_CONFIGURATION, ANTORA_SECRETS_CONFIGURATION, COMPONENT_VERSION_DESCRIPTOR,
    ComponentHandle, Error, GITIGNORE, ProjectInitDir, Resource, Result,
};
use antora_project::{
    component_name::ComponentName, component_version::ComponentVersion,
    component_version_descriptor::ComponentVersionDescriptor,
};
use relative_path::{Filename, RelativeDir, RelativeFile, create_relative_file_from_filename};
use std::{
    collections::HashMap,
    fs::{File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, Mutex},
};

static COMPONENT_VERSION_DESCRIPTOR_FILENAME: LazyLock<Filename> =
    LazyLock::new(|| Filename::try_from(COMPONENT_VERSION_DESCRIPTOR.to_owned()).unwrap());
static ANTORA_SECRETS_CONFIGURATION_FILENAME: LazyLock<Filename> =
    LazyLock::new(|| Filename::try_from(ANTORA_SECRETS_CONFIGURATION.to_owned()).unwrap());
static ANTORA_CONFIGURATION_FILENAME: LazyLock<Filename> =
    LazyLock::new(|| Filename::try_from(ANTORA_CONFIGURATION.to_owned()).unwrap());
static DEFAULT_PLAYBOOK_FILENAME: LazyLock<Filename> = LazyLock::new(|| {
    Filename::try_from(antora_project::DEFAULT_PLAYBOOK_FILENAME.to_owned()).unwrap()
});

static GITIGNORE_FILENAME: LazyLock<Filename> =
    LazyLock::new(|| Filename::try_from(GITIGNORE.to_owned()).unwrap());

#[derive(Debug)]
pub enum WriteMode {
    #[expect(unused)]
    OrOverwrite,
    IfNotExist,
    OrAppend,
}

pub struct Vfs {
    root: PathBuf,
    files: Arc<Mutex<HashMap<PathBuf, (Resource, WriteMode)>>>,
}

impl From<&ProjectInitDir> for Vfs {
    fn from(value: &ProjectInitDir) -> Self {
        Self {
            root: value.as_ref().to_owned(),
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Vfs {
    pub fn write_project_resource(&self, relative_file: RelativeFile, resource: Resource) {
        self.write_resource(relative_file, resource, WriteMode::IfNotExist);
    }

    pub(crate) fn write_resource(
        &self,
        relative_file: RelativeFile,
        resource: Resource,
        mode: WriteMode,
    ) {
        let mut pb = self.root.clone();
        let relative_file: PathBuf = relative_file.into();
        pb.push(&relative_file);

        self.files
            .lock()
            .expect("locking must succeed")
            .insert(pb, (resource, mode));
    }

    pub fn write_antora_secrets_configuration(&self, contents: String) -> &Self {
        self.write_resource(
            create_relative_file_from_filename(ANTORA_SECRETS_CONFIGURATION_FILENAME.clone()),
            Resource::TextBased(contents),
            WriteMode::IfNotExist,
        );

        self
    }

    pub fn write_antora_configuration(&self, contents: String) -> &Self {
        self.write_resource(
            create_relative_file_from_filename(ANTORA_CONFIGURATION_FILENAME.clone()),
            Resource::TextBased(contents),
            WriteMode::IfNotExist,
        );

        self
    }

    pub fn write_default_antora_playbook(&self, contents: String) -> &Self {
        self.write_antora_playbook_as(
            create_relative_file_from_filename(DEFAULT_PLAYBOOK_FILENAME.clone()),
            contents,
        )
    }

    pub fn write_component_version_descriptor(
        &self,
        mut content_source_root: RelativeDir,
        component_version_descriptor: ComponentVersionDescriptor,
    ) -> &Self {
        content_source_root.push_dir(component_version_descriptor.name().clone().into());
        self.write_resource(
            content_source_root.push_file(COMPONENT_VERSION_DESCRIPTOR_FILENAME.clone()),
            Resource::TextBased(component_version_descriptor.to_string()),
            WriteMode::IfNotExist,
        );
        self
    }

    pub fn write_antora_playbook_as(&self, relative_file: RelativeFile, contents: String) -> &Self {
        self.write_resource(
            relative_file,
            Resource::TextBased(contents),
            WriteMode::IfNotExist,
        );

        self
    }

    pub fn write_gitignore(&self, contents: String) -> &Self {
        self.write_resource(
            create_relative_file_from_filename(GITIGNORE_FILENAME.clone()),
            Resource::TextBased(contents),
            WriteMode::OrAppend,
        );

        self
    }

    pub fn register_component<'a>(
        &'a self,
        content_source_root: RelativeDir,
        component_name: ComponentName,
        component_version: ComponentVersion,
    ) -> ComponentHandle<'a> {
        let mut relative_dir = content_source_root;
        relative_dir.push_dir(component_name.clone().into());
        ComponentHandle {
            component_name,
            component_version,
            relative_dir,
            vfs: self,
        }
    }

    pub fn print(&self) {
        println!("VFS:");

        let map = self
            .files
            .lock()
            .expect("locking files in Vfs must ALWAYS succeed");
        for (p, (r, m)) in map.iter() {
            let p = p.to_string_lossy();
            let v = match r {
                Resource::Binary(data) => format!("binary-content ({}bytes)", data.len()),
                Resource::TextBased(text) => format!("text-content ({})", text.len()),
            };
            println!("\t{p} -> {v}[{m:#?}]")
        }
    }

    pub fn check_for_conflicts(&self) -> Vec<PathBuf> {
        let mut conflicting = Vec::new();
        for (pb, _) in self
            .files
            .lock()
            .expect("locking for conflict-checks must not fail")
            .iter()
            .filter(|(_p, (_, m))| matches!(m, WriteMode::IfNotExist))
        {
            if pb.exists() {
                conflicting.push(pb.into());
            }
        }
        conflicting
    }

    fn persist_resource(
        path: impl AsRef<Path>,
        resource: &Resource,
        mode: &WriteMode,
    ) -> Result<()> {
        let path = path.as_ref();

        let mut file_options = File::options();

        match mode {
            WriteMode::OrAppend => {
                file_options.append(true).create(true);
            }
            WriteMode::IfNotExist => {
                file_options.create_new(true).write(true);
            }

            WriteMode::OrOverwrite => {
                file_options.create(true).write(true);
            }
        };
        let mut file: File = file_options.open(path).map_err(Error::WritingFile)?;

        match resource {
            Resource::Binary(data) => file.write_all(data).map_err(Error::WritingFile)?,
            Resource::TextBased(text) => file
                .write_all(text.as_bytes())
                .map_err(Error::WritingFile)?,
        };

        Ok(())
    }

    pub fn persist(&self) -> Result<u64> {
        let mut number_written = 0_u64;
        for (pb, (res, mode)) in self
            .files
            .lock()
            .expect("locking for persisting must not fail")
            .iter()
        {
            create_dir_all(pb.parent().unwrap()).map_err(Error::CreatingDirs)?;
            Vfs::persist_resource(pb, res, mode)?;
            number_written += 1;
        }
        Ok(number_written)
    }
}
