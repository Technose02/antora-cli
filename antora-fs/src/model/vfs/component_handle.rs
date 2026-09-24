use std::sync::LazyLock;

use super::{ModuleHandle, Vfs};
use antora_project::{
    component_name::ComponentName, component_version::ComponentVersion, module_name::ModuleName,
};
use relative_path::{Dirname, RelativeDir};

pub(crate) static MODULES_DIRNAME: LazyLock<Dirname> =
    LazyLock::new(|| Dirname::try_from("modules".to_owned()).unwrap());

pub struct ComponentHandle<'a> {
    pub(super) component_version: ComponentVersion,
    pub(super) component_name: ComponentName,
    pub(super) relative_dir: RelativeDir,
    pub(super) vfs: &'a Vfs,
}

impl<'a> ComponentHandle<'a> {
    pub fn register_module(&'a self, module_name: ModuleName) -> ModuleHandle<'a> {
        let mut relative_dir = self.relative_dir.clone();
        relative_dir
            .push_dir(MODULES_DIRNAME.clone())
            .push_dir(module_name.clone().into());

        ModuleHandle {
            module_name,
            relative_dir,
            component_name: &self.component_name,
            component_version: &self.component_version,
            vfs: self.vfs,
        }
    }
}
