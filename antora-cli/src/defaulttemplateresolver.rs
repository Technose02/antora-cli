use antora_project::antora_configuration::ImageConfig;
use antora_project::component_version::ComponentVersion;
use init_task::{InitAssistantResults, Template, TemplateResolver, TemplateResolverError};
use init_templates::BasicTemplate;

pub struct DefaultTemplateResolver {
    valid_keys: Vec<String>,
    default_key_idx: usize,
}

impl Default for DefaultTemplateResolver {
    fn default() -> Self {
        let mut valid_keys_set = std::collections::HashSet::<String>::new();

        // set default key
        let default_key = "basic";

        // insert all new keys
        valid_keys_set.insert(String::from("basic"));

        ////

        let mut valid_keys = valid_keys_set.into_iter().collect::<Vec<String>>();
        valid_keys.sort();
        let (default_key_idx, _) = valid_keys
            .iter()
            .enumerate()
            .find(|(_, key)| *key == default_key)
            .expect("default-key should have been inserted into keys");

        Self {
            valid_keys,
            default_key_idx,
        }
    }
}

impl DefaultTemplateResolver {
    fn basic(
        &self,
        init_assistant_results: &InitAssistantResults,
        component_version: &ComponentVersion,
        include_scaffolding: bool,
    ) -> Box<BasicTemplate> {
        Box::new(BasicTemplate::new(
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
        component_version: &ComponentVersion,
        include_scaffolding: bool,
    ) -> Box<dyn Template> {
        self.basic(
            init_assistant_results,
            component_version,
            include_scaffolding,
        )
    }
    fn try_resolve(
        &self,
        init_assistant_results: &InitAssistantResults,
        component_version: &ComponentVersion,
        include_scaffolding: bool,
    ) -> Result<Box<dyn Template>, TemplateResolverError> {
        match init_assistant_results.init_template_key() {
            "basic" => Ok(self.basic(
                init_assistant_results,
                component_version,
                include_scaffolding,
            )),
            s => Err(TemplateResolverError::invalid_init_template_key(s)),
        }
    }

    fn default_key(&self) -> &str {
        self.valid_keys[self.default_key_idx].as_str()
    }

    fn valid_keys(&self) -> &[String] {
        self.valid_keys.as_slice()
    }

    fn default_image_config(&self) -> antora_project::antora_configuration::ImageConfig {
        ImageConfig {
            antora_image: "<ANTORA_IMAGE_PATH>".to_owned(),
            version_tag: "<ANTORA_IMAGE_VERSION_TAG>".to_owned(),
        }
    }
}
