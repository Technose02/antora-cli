use super::{InitAssistantResults, Template, Vfs};
use antora_fs::{ANTORA_BUILD_DIR, ANTORA_CACHE_DIR};
use antora_project::{
    antora_configuration::{AntoraConfiguration, ImageConfig, PlaybookConfig},
    antora_playbook::{
        AntoraExtensionBuilder, AntoraPlaybook, AntoraPlaybookBuilder, AntoraSectionBuilder,
        AsciidocSectionBuilder, AttributeValue, ContentSectionBuilder, ContentSourceBuilder,
        LogBuilder, LogLevel, OutputSectionBuilder, RuntimeSectionBuilder, SiteSectionBuilder,
        UiBundleBuilder, UiSectionBuilder,
    },
    component_name::ComponentName,
    component_version::ComponentVersion,
    component_version_descriptor::{
        self, ComponentVersionDescriptor,
        extension_config::collector_extension::scan::{ScanValueArrayBuilder, ScanValueMap},
    },
    module_name::{ModuleName, ROOT_MODULE},
    resource_id::{ResourceId, ResourceIdDetailLevel},
};
use relative_path::RelativeDir;

mod antora_assembler_pdf_yml;
mod index_adoc;
mod scaffolding;
//mod pdf_theme_yml;

pub struct Basic {
    init_assistant_results: InitAssistantResults,
    component_version: ComponentVersion,
    cached_component_version_descriptor: Option<ComponentVersionDescriptor>,
    cached_playbook: Option<AntoraPlaybook>,
    include_scaffolding: bool,
}

impl Basic {
    pub fn new(
        init_assistant_results: &InitAssistantResults,
        component_version: ComponentVersion,
        include_scaffolding: bool,
    ) -> Self {
        Self {
            init_assistant_results: init_assistant_results.clone(),
            component_version,
            cached_component_version_descriptor: None,
            cached_playbook: None,
            include_scaffolding,
        }
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
                    .attribute("kroki-server-url", "https://kroki.io")
                    .attribute("diagram-server-url", "https://kroki.io")
                    .attribute("diagram-server-type", "kroki_io")
                    .attribute("kroki-fetch-diagram", true)
                    .build(),
            )
            .build()
    }

    fn create_component_version_descriptor(
        content_source_root: RelativeDir,
        component_name: impl Into<ComponentName>,
        component_title: impl Into<String>,
        component_version: &ComponentVersion,
        collector_extension: bool,
    ) -> ComponentVersionDescriptor {
        let component_name = component_name.into();
        let mut component_version_descriptor =
            ComponentVersionDescriptor::basic(component_name.clone());
        component_version_descriptor.with_title(component_title);
        component_version_descriptor.with_version(component_version);

        if collector_extension {
            // create collector-config
            let project_dir = format!(
                "{}{}",
                content_source_root,
                "/..".repeat(content_source_root.count())
            );
            let component_root_dir = format!("{content_source_root}/{component_name}",);
            let extension_config = {
                use component_version_descriptor::extension_config::collector_extension::CollectorConfigArrayBuilder;

                component_version_descriptor::extension_config::ExtensionConfig::default()
                    .collector(
                        CollectorConfigArrayBuilder::default()
                            .scan(
                                ScanValueArrayBuilder::default()
                                    .push(
                                        ScanValueMap::new(project_dir.as_str())
                                            .files("antora-playbook.yml")
                                            .into("modules/scaffolding/examples/collected"),
                                    )
                                    .push(
                                        ScanValueMap::new(component_root_dir.as_str())
                                            .files("antora.yml")
                                            .into("modules/scaffolding/examples/collected"),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
            };
            component_version_descriptor.with_ext(extension_config);
        }

        component_version_descriptor
    }
}

impl Template for Basic {
    fn process(&mut self, vfs: &Vfs, pdf_target: bool) {
        // create start_path for content
        let mut content_source_start_path =
            self.init_assistant_results.content_source_root().clone();
        content_source_start_path
            .push_dir(self.init_assistant_results.component_name().clone().into());

        // create component_version_descriptor
        let mut component_version_descriptor = Self::create_component_version_descriptor(
            self.init_assistant_results.content_source_root(),
            self.init_assistant_results.component_name().clone(),
            self.init_assistant_results.component_title(),
            &self.component_version,
            true,
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
        let start_page_id = module.write_page(
            index_adoc::relative_file(),
            index_adoc::content(
                &self.init_assistant_results,
                &self.component_version,
                self.include_scaffolding,
            ),
        );

        let nav_file_path = module.write_nav(format!(
            "* xref:{}[Start]",
            start_page_id.create_simplified(ResourceIdDetailLevel::Module)
        ));

        component_version_descriptor.with_nav(nav_file_path);

        if self.include_scaffolding {
            let scaffolding_module = component.register_module(
                ModuleName::try_from("scaffolding").expect("scaffolding is a valid ModuleName"),
            );
            scaffolding_module.init_all_family_directories();
            let index_adoc_id = scaffolding_module.write_page(
                scaffolding::index_adoc_relative_file(),
                scaffolding::index_adoc_content(
                    &self.init_assistant_results,
                    &self.component_version,
                ),
            );
            let antora_concepts_adoc_id = scaffolding_module.write_page(
                scaffolding::antora_concepts_adoc_relative_file(),
                scaffolding::antora_concepts_adoc_content(&self.init_assistant_results),
            );
            let asciidoc_intro_adoc_id = scaffolding_module.write_page(
                scaffolding::asciidoc_intro_adoc_relative_file(),
                scaffolding::asciidoc_intro_adoc_content(),
            );
            let diagrams_adoc_id = scaffolding_module.write_page(
                scaffolding::diagrams_adoc_relative_file(),
                scaffolding::diagrams_adoc_content(),
            );
            scaffolding_module.write_example(
                scaffolding::antora_yml_placeholder_relative_file(),
                scaffolding::antora_yml_placeholder_content(),
            );
            scaffolding_module.write_example(
                scaffolding::antora_playbook_placeholder_relative_file(),
                scaffolding::antora_playbook_placeholder_content(),
            );
            scaffolding_module.write_image(
                scaffolding::abbildung_svg_relative_file(),
                scaffolding::abbildung_svg_content(),
            );
            scaffolding_module.write_image(
                scaffolding::flowchart_mmd_relative_file(),
                scaffolding::flowchart_mmd_content(),
            );
            scaffolding_module.write_image(
                scaffolding::sequence_puml_relative_file(),
                scaffolding::sequence_puml_content(),
            );
            scaffolding_module.write_partial(
                scaffolding::sample_table_relative_file(),
                scaffolding::sample_table_content(),
            );
            let scaffolding_nav_file_path = scaffolding_module.write_nav(format!(
                r#"* xref:{}[Scaffolding]
** xref:{}[Antora-Konzepte]
** xref:{}[AsciiDoc-Intro]
*** xref:{}[Diagramme]"#,
                index_adoc_id, antora_concepts_adoc_id, asciidoc_intro_adoc_id, diagrams_adoc_id
            ));

            component_version_descriptor.with_nav(scaffolding_nav_file_path);
        }

        self.cached_playbook = Some(Self::create_playbook(
            start_page_id,
            self.init_assistant_results.playbook_site_title(),
            content_source_start_path,
            pdf_target,
            true,
        ));

        self.cached_component_version_descriptor = Some(component_version_descriptor);

        if pdf_target {
            vfs.write_project_resource(
                antora_assembler_pdf_yml::relative_file(),
                antora_assembler_pdf_yml::content(),
            );
            //vfs.write_project_resource(pdf_theme_yml::relative_file(), pdf_theme_yml::content());
        }
    }

    fn get_antora_configuration(&self) -> AntoraConfiguration {
        AntoraConfiguration {
            playbook: PlaybookConfig::default(),
            antora_image: ImageConfig::default(),
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
