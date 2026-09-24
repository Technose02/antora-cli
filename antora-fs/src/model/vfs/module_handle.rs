use super::{Vfs, component_handle::MODULES_DIRNAME, vfsimpl::WriteMode};
use crate::Resource;
use antora_project::{
    component_name::ComponentName, component_version::ComponentVersion, families::Families,
    module_name::ModuleName, resource_id::ResourceId,
};
use relative_path::{Filename, RelativeDir, RelativeFile};
use std::sync::LazyLock;
pub(crate) static NAV_FILENAME: LazyLock<Filename> =
    LazyLock::new(|| Filename::try_from("nav.adoc".to_owned()).unwrap());

pub struct ModuleHandle<'a> {
    pub(super) module_name: ModuleName,
    pub(super) relative_dir: RelativeDir,
    pub(super) component_name: &'a ComponentName,
    pub(super) component_version: &'a ComponentVersion,
    pub(super) vfs: &'a Vfs,
}

impl<'a> ModuleHandle<'a> {
    pub fn write_nav(&self, contents: String) -> RelativeFile {
        let relative_file = self.relative_dir.clone().push_file(NAV_FILENAME.clone());

        self.vfs.write_resource(
            relative_file,
            Resource::TextBased(contents),
            WriteMode::IfNotExist,
        );

        let mut relative_dir_for_output = RelativeDir::from(MODULES_DIRNAME.clone());
        relative_dir_for_output.push_dir(self.module_name.clone().into());
        relative_dir_for_output.push_file(NAV_FILENAME.clone())
    }

    fn write_familily_resource(
        &self,
        family: Families,
        resource_file: RelativeFile,
        resource_contents: Resource,
    ) -> ResourceId {
        let mut relative_dir = self.relative_dir.clone();

        relative_dir.push_dir(family.clone().into());
        let relative_file = relative_dir.append(resource_file.clone());

        self.vfs.write_resource(
            relative_file.clone(),
            resource_contents,
            WriteMode::IfNotExist,
        );

        let mut id = ResourceId::new(resource_file.clone());
        id = id
            .with_family(family)
            .with_module(self.module_name.clone())
            .with_component_name(self.component_name.clone());
        if !self.component_version.is_empty() {
            id = id.with_component_version(self.component_version.clone());
        }
        id
    }

    pub fn write_attachment(
        &self,
        resource_file: RelativeFile,
        resource_contents: Resource,
    ) -> ResourceId {
        self.write_familily_resource(Families::Attachments, resource_file, resource_contents)
    }

    pub fn write_example(
        &self,
        resource_file: RelativeFile,
        resource_contents: Resource,
    ) -> ResourceId {
        self.write_familily_resource(Families::Examples, resource_file, resource_contents)
    }

    pub fn write_image(
        &self,
        resource_file: RelativeFile,
        resource_contents: Resource,
    ) -> ResourceId {
        self.write_familily_resource(Families::Images, resource_file, resource_contents)
    }

    pub fn write_page(
        &self,
        resource_file: RelativeFile,
        resource_contents: Resource,
    ) -> ResourceId {
        self.write_familily_resource(Families::Pages, resource_file, resource_contents)
    }

    pub fn init_all_family_directories(&self) -> &Self {
        let filename = RelativeFile::try_from(".gitkeep").expect("this is a correct relative file");
        self.write_attachment(filename.clone(), Resource::Binary(Vec::new()));
        self.write_example(filename.clone(), Resource::Binary(Vec::new()));
        self.write_image(filename.clone(), Resource::Binary(Vec::new()));
        self.write_page(filename.clone(), Resource::Binary(Vec::new()));
        self.write_partial(filename, Resource::Binary(Vec::new()));
        self
    }

    pub fn write_partial(
        &self,
        resource_file: RelativeFile,
        resource_contents: Resource,
    ) -> ResourceId {
        self.write_familily_resource(Families::Partials, resource_file, resource_contents)
    }
}
