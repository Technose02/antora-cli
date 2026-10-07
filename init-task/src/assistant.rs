use crate::TemplateResolver;
use antora_fs::DEFAULT_CONTENT_SOURCE_ROOT;
use antora_project::{Error as AntoraTypesError, component_name::ComponentName};
use relative_path::{Dirname, RelativeDir, create_relative_dir_from_dirname};
use std::{io::stdin, process::exit, sync::LazyLock};

pub static DEFAULT_CONTENT_SOURCE_ROOT_DIRNAME: LazyLock<Dirname> =
    LazyLock::new(|| Dirname::try_from(DEFAULT_CONTENT_SOURCE_ROOT.to_owned()).unwrap());

pub enum ValidationResult {
    Valid,
    Invalid(String),
    Empty,
}

enum EmptyRule {
    Cancel,
    Default(String),
    Valid,
}

fn prompt_for_value(
    prompt: &str,
    validate: impl Fn(&String) -> ValidationResult,
    empty_rule: EmptyRule,
) -> String {
    let mut buf = String::new();

    println!("{prompt}");
    loop {
        if stdin().read_line(&mut buf).is_ok() {
            buf = buf.trim_end().into();

            match validate(&buf) {
                ValidationResult::Valid => return buf,
                ValidationResult::Empty => match empty_rule {
                    EmptyRule::Cancel => {
                        println!("cancelled");
                        exit(1);
                    }
                    EmptyRule::Default(s) => return s,
                    EmptyRule::Valid => return String::new(),
                },
                ValidationResult::Invalid(msg) => {
                    println!("{msg}");
                    println!("  Please try again (or enter nothing to cancel):");
                    buf.clear();
                }
            }
        } else {
            eprintln!("error reading from stdin");
            exit(1);
        }
    }
}

fn get_component_name_from_prompt_or_exit() -> ComponentName {
    let validated = prompt_for_value(
        "please enter a name for your component (or nothing to cancel):",
        |content_source_name_candidate| {
            if content_source_name_candidate.is_empty() {
                ValidationResult::Empty
            } else if let Err(AntoraTypesError::InvalidComponentName(hint)) =
                ComponentName::try_from(content_source_name_candidate.clone())
            {
                ValidationResult::Invalid(hint)
            } else {
                ValidationResult::Valid
            }
        },
        EmptyRule::Cancel,
    );
    ComponentName::try_from(validated).unwrap()
}

fn get_component_title_from_prompt_or_exit(component_name: &ComponentName) -> String {
    prompt_for_value(
        &format!("please enter a title for component '{component_name}' (or nothing to cancel):"),
        |content_source_title_candidate| {
            if content_source_title_candidate.is_empty() {
                ValidationResult::Empty
            } else {
                ValidationResult::Valid
            }
        },
        EmptyRule::Cancel,
    )
}

fn get_optional_playbook_title_from_prompt() -> String {
    prompt_for_value(
        "enter a different title for your playbook (or leave empty to use the component's title for the playbook, too):",
        |playbook_title_candidate| {
            if playbook_title_candidate.is_empty() {
                ValidationResult::Empty
            } else {
                ValidationResult::Valid
            }
        },
        EmptyRule::Valid,
    )
}

pub struct InitAssistant<'a> {
    interactive: bool,
    provided_content_source_root: Option<&'a String>,
    provided_component_name: Option<&'a String>,
    provided_component_title: Option<&'a String>,
    provided_playbook_site_title: Option<&'a String>,
    provided_init_template_key: Option<&'a String>,
}

impl<'a, 'e> InitAssistant<'a>
where
    'e: 'a,
{
    pub fn new(interactive: bool) -> Self {
        Self {
            interactive,
            provided_content_source_root: None,
            provided_component_name: None,
            provided_component_title: None,
            provided_playbook_site_title: None,
            provided_init_template_key: None,
        }
    }

    pub fn with_provided_content_source_root(
        &mut self,
        provided_content_source_root: Option<&'e String>,
    ) -> &mut Self {
        self.provided_content_source_root = provided_content_source_root;
        self
    }

    pub fn with_provided_component_name(
        &mut self,
        provided_component_name: Option<&'e String>,
    ) -> &mut Self {
        self.provided_component_name = provided_component_name;
        self
    }

    pub fn with_provided_component_title(
        &mut self,
        provided_component_title: Option<&'e String>,
    ) -> &mut Self {
        self.provided_component_title = provided_component_title;
        self
    }

    pub fn with_provided_playbook_site_title(
        &mut self,
        provided_playbook_site_title: Option<&'e String>,
    ) -> &mut Self {
        self.provided_playbook_site_title = provided_playbook_site_title;
        self
    }

    pub fn with_provided_init_template_key(
        &mut self,
        provided_init_template_key: Option<&'e String>,
    ) -> &mut Self {
        self.provided_init_template_key = provided_init_template_key;
        self
    }

    fn create_invalid_init_template_key_error_message(
        template_resolver: &dyn TemplateResolver,
        init_template_key: impl AsRef<str>,
    ) -> String {
        format!(
            r#"error: init-template-key '{}' is not valid

Valid init-template-keys are:
{}"#,
            init_template_key.as_ref(),
            template_resolver.valid_keys().join(",")
        )
    }

    pub fn process_exitting_eventually(
        self,
        template_resolver: &dyn TemplateResolver,
    ) -> InitAssistantResults {
        // first: check if there was an invalid template-key provided and exit eventually
        if let Some(key) = self.provided_init_template_key
            && !template_resolver.valid_keys().contains(key)
        {
            eprintln!(
                "{}",
                Self::create_invalid_init_template_key_error_message(template_resolver, key)
            );
            exit(1);
        }

        // process component_name
        let component_name = match (self.interactive, self.provided_component_name) {
            (false, None) => {
                eprintln!("error: no component-name provided");
                exit(1);
            }
            (_, Some(component_name)) => {
                if let Ok(component_name) = ComponentName::try_from(component_name.to_owned()) {
                    component_name
                } else {
                    eprintln!("error: invalid component-name");
                    exit(1);
                }
            }
            (true, None) => get_component_name_from_prompt_or_exit(),
        };

        // process component_title
        let component_title = match (self.interactive, self.provided_component_title) {
            (false, None) => {
                eprintln!("error: no component-title provided");
                exit(1);
            }
            (_, Some(component_title)) => component_title.to_owned(),
            (true, None) => get_component_title_from_prompt_or_exit(&component_name),
        };

        // process content_source_root
        let content_source_root = if let Some(content_source_root) =
            self.provided_content_source_root
        {
            if let Ok(content_source_root) = RelativeDir::try_from(content_source_root.as_str()) {
                content_source_root
            } else {
                eprintln!("error: provided content-source-root is invalid");
                exit(1);
            }
        } else {
            create_relative_dir_from_dirname(DEFAULT_CONTENT_SOURCE_ROOT_DIRNAME.clone())
        };

        let playbook_site_title = match (self.interactive, self.provided_playbook_site_title) {
            (false, None) => component_title.clone(),
            (_, Some(playbook_site_title)) => playbook_site_title.clone(),
            (true, None) => {
                let from_prompt = get_optional_playbook_title_from_prompt();
                if from_prompt.is_empty() {
                    component_title.clone()
                } else {
                    from_prompt
                }
            }
        };

        let init_template_key = self.provided_init_template_key.map(String::to_owned).unwrap_or_else(|| {
            prompt_for_value(
                &format!(
                    "please enter the key of the init-template to use (or nothing to default to init-template '{}'):", template_resolver.default_key()
                ),
                |init_template_key_candidate| {
                    if init_template_key_candidate.is_empty() {
                        ValidationResult::Empty
                    } else {
                        if template_resolver.valid_keys().contains(init_template_key_candidate) {
                            ValidationResult::Valid
                        } else {
                            ValidationResult::Invalid(
                                Self::create_invalid_init_template_key_error_message(template_resolver, init_template_key_candidate))
                        }
                    }
                },
                EmptyRule::Default(template_resolver.default_key().to_owned()),
            )
        });

        InitAssistantResults {
            component_name,
            component_title,
            content_source_root,
            playbook_site_title,
            init_template_key,
        }
    }
}

#[derive(Clone)]
pub struct InitAssistantResults {
    component_name: ComponentName,
    component_title: String,
    content_source_root: RelativeDir,
    playbook_site_title: String,
    init_template_key: String,
}

impl InitAssistantResults {
    pub fn component_name(&self) -> &ComponentName {
        &self.component_name
    }
    pub fn component_title(&self) -> &str {
        self.component_title.as_str()
    }
    pub fn content_source_root(&self) -> RelativeDir {
        self.content_source_root.clone()
    }
    pub fn playbook_site_title(&self) -> &str {
        self.playbook_site_title.as_str()
    }
    pub fn init_template_key(&self) -> &str {
        self.init_template_key.as_str()
    }
}
