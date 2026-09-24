use antora_project::component_version::ComponentVersion;
mod application;
mod basictemplate;
pub mod tasks;
use basictemplate::Basic;
use init_task::Template;
use init_task::{InitAssistantResults, TemplateResolver, TemplateResolverError};

pub struct DefaultTemplateResolver;
impl DefaultTemplateResolver {
    fn basic(
        &self,
        init_assistant_results: &InitAssistantResults,
        component_version: ComponentVersion,
        include_scaffolding: bool,
    ) -> Box<Basic> {
        Box::new(Basic::new(
            init_assistant_results,
            component_version,
            include_scaffolding,
        ))
    }
}

impl TemplateResolver for DefaultTemplateResolver {
    fn default(
        &self,
        init_assistant_results: &InitAssistantResults,
        component_version: ComponentVersion,
        include_scaffolding: bool,
    ) -> Box<dyn Template> {
        self.basic(
            init_assistant_results,
            component_version,
            include_scaffolding,
        )
    }
    fn resolve(
        &self,
        key: &str,
        init_assistant_results: &InitAssistantResults,
        component_version: ComponentVersion,
        include_scaffolding: bool,
    ) -> Result<Box<dyn Template>, TemplateResolverError> {
        match key {
            "basic" => Ok(self.basic(
                init_assistant_results,
                component_version,
                include_scaffolding,
            )),
            s => Err(TemplateResolverError::invalid_template_key(s)),
        }
    }
}

pub fn run_from_env_args(template_resolver: Box<dyn TemplateResolver>) {
    application::run_cli(std::env::args_os(), template_resolver)
}

pub fn run_from_args<I, T, R>(args_iter: I, template_resolver: Box<dyn TemplateResolver>)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    application::run_cli(args_iter, template_resolver)
}

#[macro_export]
macro_rules! run_cli {
    ($template_resolver: ident) => {{ antora_cli::run_from_env_args($template_resolver) }};
    ($args_iter: ident, $template_resolver: ident) => {{ antora_cli::run_from_args($args_iter, $template_resolver) }};
}
