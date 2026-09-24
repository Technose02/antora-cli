use regex::Regex;
use std::sync::LazyLock;

pub mod antora_configuration;
pub mod antora_playbook;
pub mod antora_secrets;
pub mod component_name;
pub mod component_version;
pub mod component_version_descriptor;
pub mod families;
pub mod module_name;
pub mod resource_id;

fn is_valid_component_or_module_name(component_name: impl AsRef<str>) -> bool {
    static VALID_COMPONENT_OR_MODULE_NAME: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new("[0-9a-z]+[0-9a-z\\-]*")
            .expect("static pattern for valid component-name must compile")
    });

    let component_name = component_name.as_ref();
    matches!(VALID_COMPONENT_OR_MODULE_NAME.find(component_name),Some(matched) if matched.as_str() == component_name && !component_name.ends_with("-"))
}
