use antora_fs::{ANTORA_BUILD_DIR, ANTORA_CACHE_DIR, ANTORA_SECRETS_CONFIGURATION, Vfs};
use antora_project::{
    antora_configuration::{AntoraConfiguration, PlaybookConfig},
    antora_playbook::{
        AntoraExtensionBuilder, AntoraPlaybook, AntoraPlaybookBuilder, AntoraSectionBuilder,
        AsciidocSectionBuilder, AttributeValue, ContentSectionBuilder, ContentSourceBuilder,
        LogBuilder, LogLevel, OutputSectionBuilder, RuntimeSectionBuilder, SiteSectionBuilder,
        UiBundleBuilder, UiSectionBuilder,
    },
    component_name::ComponentName,
    component_version::ComponentVersion,
    component_version_descriptor::ComponentVersionDescriptor,
    module_name::ROOT_MODULE,
    resource_id::{ResourceId, ResourceIdDetailLevel},
};
use init_task::{InitAssistantResults, Template, TemplateResolver};
use relative_path::RelativeDir;

const DEFAULT_KROKI_SERVER_URL: &str = "https://kroki.io/";

mod antora_assembler_pdf_yml;
mod index_adoc;

pub struct BasicTemplate {
    init_assistant_results: InitAssistantResults,
    component_version: ComponentVersion,
    cached_component_version_descriptor: Option<ComponentVersionDescriptor>,
    cached_playbook: Option<AntoraPlaybook>,
}

impl BasicTemplate {
    pub fn new(
        init_assistant_results: &InitAssistantResults,
        component_version: &ComponentVersion,
    ) -> Self {
        Self {
            init_assistant_results: init_assistant_results.clone(),
            component_version: component_version.clone(),
            cached_component_version_descriptor: None,
            cached_playbook: None,
        }
    }

    pub fn with_cached_component_version_descriptor(
        &mut self,
        cached_component_version_descriptor: ComponentVersionDescriptor,
    ) -> &mut Self {
        self.cached_component_version_descriptor = Some(cached_component_version_descriptor);
        self
    }

    pub fn with_cached_playbook(&mut self, cached_playbook: AntoraPlaybook) -> &mut Self {
        self.cached_playbook = Some(cached_playbook);
        self
    }

    fn create_playbook(
        site_start_page: ResourceId,
        site_title: &str,
        content_source_start_path: RelativeDir,
        pdf_target: bool,
        collector_extension: bool,
    ) -> AntoraPlaybook {
        let antora_section = {
            let mut antora_section_builder = {
                let b = AntoraSectionBuilder::new()
                    .extension(AntoraExtensionBuilder::new("@antora/lunr-extension").build());
                if collector_extension {
                    b.extension(AntoraExtensionBuilder::new("@antora/collector-extension").build())
                } else {
                    b
                }
            };

            if pdf_target {
                let pdf_extension_builder = AntoraExtensionBuilder::new("@antora/pdf-extension")
                    .config_file(antora_assembler_pdf_yml::relative_file())
                    .build();
                antora_section_builder = antora_section_builder.extension(pdf_extension_builder);
            }
            antora_section_builder.build()
        };

        AntoraPlaybookBuilder::new()
            .site(
                SiteSectionBuilder::new()
                    .start_page(site_start_page.create_simplified(ResourceIdDetailLevel::default()))
                    .title(site_title)
                    .url("http://www.example.org")
                    .build(),
            )
            .content(
                ContentSectionBuilder::new()
                    .source(
                        ContentSourceBuilder::new()
                            .branch("main")
                            .branch("master")
                            .start_path(content_source_start_path)
                            .url(".")
                            .build(),
                    )
                    .build(),
            )
            .ui(UiSectionBuilder::new()
                .bundle(
                    UiBundleBuilder::new()
                        .url("/ui/default/ui-bundle.zip")
                        .snapshot(true)
                        .build(),
                )
                .supplemental_files("/ui/default/overrides")
                .build())
            .runtime(
                RuntimeSectionBuilder::new()
                    .cache_dir(ANTORA_CACHE_DIR)
                    .log(LogBuilder::new().level(LogLevel::All).build())
                    .build(),
            )
            .antora(antora_section)
            .output(
                OutputSectionBuilder::new()
                    .dir(format!("{ANTORA_BUILD_DIR}/site"))
                    .clean(true)
                    .build(),
            )
            .asciidoc(
                AsciidocSectionBuilder::new()
                    .extension("asciidoctor-kroki")
                    .attribute("antora-build", "ok")
                    .attribute("xrefstyle", "short")
                    .attribute("hide-secrets", "on")
                    .attribute("arc42help", true)
                    .attribute("toc-title", "Inhaltsverzeichnis")
                    .attribute("toc", AttributeValue::None)
                    .attribute("caution-caption", "Achtung")
                    .attribute("important-caption", "Wichtig")
                    .attribute("note-caption", "Hinweis")
                    .attribute("tip-caption", "Tip")
                    .attribute("warning-caption", "Warnung")
                    .attribute("appendix-caption", "Anhang")
                    .attribute("example-caption", "Beispiel")
                    .attribute("figure-caption", "Abbildung")
                    .attribute("table-caption", "Tabelle")
                    .attribute("collapsible-arc42-help", "use")
                    .attribute("kroki-server-url", DEFAULT_KROKI_SERVER_URL)
                    .attribute("diagram-server-url", DEFAULT_KROKI_SERVER_URL)
                    .attribute("diagram-server-type", "kroki_io")
                    .attribute("kroki-fetch-diagram", true)
                    .build(),
            )
            .build()
    }

    fn create_component_version_descriptor(
        component_name: impl Into<ComponentName>,
        component_title: impl Into<String>,
        component_version: &ComponentVersion,
    ) -> ComponentVersionDescriptor {
        let component_name = component_name.into();
        let mut component_version_descriptor =
            ComponentVersionDescriptor::basic(component_name.clone());
        component_version_descriptor.with_title(component_title);
        component_version_descriptor.with_version(component_version);

        component_version_descriptor
    }
}

impl Template for BasicTemplate {
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

    fn process(&mut self, vfs: &Vfs, pdf_target: bool) {
        // create start_path for content
        let mut content_source_start_path =
            self.init_assistant_results.content_source_root().clone();
        content_source_start_path
            .push_dir(self.init_assistant_results.component_name().clone().into());

        // create component_version_descriptor
        let mut component_version_descriptor = Self::create_component_version_descriptor(
            self.init_assistant_results.component_name().clone(),
            self.init_assistant_results.component_title(),
            &self.component_version,
        );

        // create and register component to create module
        let component = vfs.register_component(
            self.init_assistant_results.content_source_root(),
            component_version_descriptor.name().clone(),
            component_version_descriptor.version().clone(),
        );

        // create and register module to create content
        let module = component.register_module(ROOT_MODULE.clone());
        module.init_all_family_directories();

        // write content to registered module
        let start_page_id = module.write_page(index_adoc::resource_file(
            &self.init_assistant_results,
            &self.component_version,
        ));

        let nav_file_path = module.write_nav(format!(
            "* xref:{}[Start]",
            start_page_id.create_simplified(ResourceIdDetailLevel::Module)
        ));

        component_version_descriptor.with_nav(nav_file_path);

        self.cached_playbook = Some(Self::create_playbook(
            start_page_id,
            self.init_assistant_results.playbook_site_title(),
            content_source_start_path,
            pdf_target,
            true,
        ));

        self.cached_component_version_descriptor = Some(component_version_descriptor);

        if pdf_target {
            vfs.write_project_resource((
                antora_assembler_pdf_yml::relative_file(),
                antora_assembler_pdf_yml::content(),
            ));
        }
    }

    fn get_antora_configuration(
        &self,
        template_resolver: &dyn TemplateResolver,
    ) -> AntoraConfiguration {
        AntoraConfiguration {
            playbook: PlaybookConfig::default(),
            antora_image: template_resolver.default_image_config(),
            confluence: None,
        }
    }

    fn get_antora_playbook(&self) -> Option<AntoraPlaybook> {
        self.cached_playbook.clone()
    }

    fn get_component_version_descriptor(&self) -> Option<ComponentVersionDescriptor> {
        self.cached_component_version_descriptor.clone()
    }
}
