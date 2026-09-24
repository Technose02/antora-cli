use super::{InitAssistantResults, Vfs};
use antora_fs::{ANTORA_BUILD_DIR, ANTORA_CACHE_DIR, ANTORA_SECRETS_CONFIGURATION};
use antora_project::{
    antora_configuration::AntoraConfiguration, antora_playbook::AntoraPlaybook,
    component_version::ComponentVersion, component_version_descriptor::ComponentVersionDescriptor,
};

mod basic;
use basic::Basic;

pub trait Template {
    /// writes its contents into the assigned vfs
    /// based on the results provided by the InitAssistant
    /// SHOULD NOT WRITE (files will be overwritten by init-task)
    /// - .gitignore
    /// - antora.yaml
    /// - playbook-file
    /// - antora.toml
    /// - antora-secrets.toml
    fn process(&mut self, vfs: &Vfs, pdf_target: bool);

    /// returns the content to write into the .gitignore
    fn get_gitignore_content(&self) -> String {
        format!(
            r#"
# rules for antora
/**/.venv
{ANTORA_CACHE_DIR}/
{ANTORA_BUILD_DIR}/
{ANTORA_SECRETS_CONFIGURATION}
"#,
        )
    }

    /// returns the structure to write as antora.yaml
    fn get_antora_configuration(&self) -> AntoraConfiguration;

    /// returns the structure to write as playbook
    fn get_antora_playbook(&self) -> Option<AntoraPlaybook> {
        None
    }

    /// returns the structure to write as component-version-descriptor of
    /// the component if the template creates any
    fn get_component_version_descriptor(&self) -> Option<ComponentVersionDescriptor> {
        None
    }
}

pub enum Templates {
    Basic,
}

impl Templates {
    pub fn create(
        &self,
        init_assistant_results: &InitAssistantResults,
        component_version: ComponentVersion,
        include_scaffolding: bool,
    ) -> Box<dyn Template> {
        match self {
            Templates::Basic => Box::new(Basic::new(
                init_assistant_results,
                component_version,
                include_scaffolding,
            )),
        }
    }
}
