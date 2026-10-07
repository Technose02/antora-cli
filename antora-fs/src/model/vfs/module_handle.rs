use super::{Vfs, component_handle::MODULES_DIRNAME, vfsimpl::WriteMode};
use crate::{Resource, ResourceFile};
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
            (relative_file, Resource::TextBased(contents)),
            WriteMode::IfNotExist,
        );

        let mut relative_dir_for_output = RelativeDir::from(MODULES_DIRNAME.clone());
        relative_dir_for_output.push_dir(self.module_name.clone().into());
        relative_dir_for_output.push_file(NAV_FILENAME.clone())
    }

    fn write_familily_resource(
        &self,
        family: Families,
        resource_file: impl ResourceFile,
    ) -> ResourceId {
        let mut relative_dir = self.relative_dir.clone();

        relative_dir.push_dir(family.clone().into());

        let (file, content) = resource_file.split();
        self.vfs.write_resource(
            (relative_dir.append(file.clone()), content),
            WriteMode::IfNotExist,
        );

        let mut id = ResourceId::new(file);
        id = id
            .with_family(family)
            .with_module(self.module_name.clone())
            .with_component_name(self.component_name.clone());
        if !self.component_version.is_empty() {
            id = id.with_component_version(self.component_version.clone());
        }
        id
    }

    pub fn write_attachment(&self, resource_file: impl ResourceFile) -> ResourceId {
        self.write_familily_resource(Families::Attachments, resource_file)
    }

    pub fn write_example(&self, resource_file: impl ResourceFile) -> ResourceId {
        self.write_familily_resource(Families::Examples, resource_file)
    }

    pub fn write_image(&self, resource_file: impl ResourceFile) -> ResourceId {
        self.write_familily_resource(Families::Images, resource_file)
    }

    pub fn write_page(&self, resource_file: impl ResourceFile) -> ResourceId {
        self.write_familily_resource(Families::Pages, resource_file)
    }

    pub fn init_all_family_directories(&self) -> &Self {
        let resource_file = (
            RelativeFile::try_from(".gitkeep").expect("this is a correct relative file"),
            Vec::new().into(),
        );
        self.write_attachment(resource_file.clone());
        self.write_example(resource_file.clone());
        self.write_image(resource_file.clone());
        self.write_page(resource_file.clone());
        self.write_partial(resource_file);
        self
    }

    pub fn write_partial(&self, resource_file: impl ResourceFile) -> ResourceId {
        self.write_familily_resource(Families::Partials, resource_file)
    }
}
