use antora_fs::Vfs;
use antora_project::{
    antora_configuration::AntoraConfiguration,
    antora_playbook::AntoraPlaybook,
    component_version_descriptor::{ComponentVersion, ComponentVersionDescriptor},
};

mod error;
pub use error::Error as TemplateResolverError;
use error::Result;

mod assistant;
pub use assistant::InitAssistantResults;

mod inittask;
pub use inittask::InitTask;

pub struct InitArgs<'cli> {
    pub project_dir: Option<&'cli String>,
    pub non_interactive_flag: bool,
    pub include_scaffolding: bool,
    pub provided_docs_dir: Option<&'cli String>,
    pub provided_component_name: Option<&'cli String>,
    pub provided_component_title: Option<&'cli String>,
    pub provided_component_version: Option<&'cli String>,
    pub provided_playbook_site_title: Option<&'cli String>,
    pub template_key: Option<&'cli String>,
    pub export_pdf: bool,
}

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
    fn get_gitignore_content(&self) -> String;

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

pub trait TemplateResolver {
    fn resolve(
        &self,
        key: &str,
        init_assistant_results: &InitAssistantResults,
        component_version: ComponentVersion,
        include_scaffolding: bool,
    ) -> Result<Box<dyn Template>>;
    fn default(
        &self,
        init_assistant_results: &InitAssistantResults,
        component_version: ComponentVersion,
        include_scaffolding: bool,
    ) -> Box<dyn Template>;
    fn valid_keys(&self) -> &[&str];
}
