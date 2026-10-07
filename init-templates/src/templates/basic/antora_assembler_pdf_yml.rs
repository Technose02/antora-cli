use antora_fs::{ANTORA_BUILD_DIR, Resource};
use relative_path::RelativeFile;

pub(super) fn relative_file() -> RelativeFile {
    "antora-assembler-pdf.yml"
        .try_into()
        .expect("antora-assembler-pdf.yml is a valid relative-path to a file")
}

pub(super) fn content() -> Resource {
    Resource::TextBased(format!(
        r#"component_version_filter:
  names: ['*']
asciidoc:
  attributes:
    backend-pdf: true
    allow-uri-read: true
    kroki-fetch-diagram: false
    source-highlighter: coderay
    #pdf-theme: pdf_theme.yml
build:
  dir: {ANTORA_BUILD_DIR}
  clean: true
  command: asciidoctor-pdf -r asciidoctor-kroki
assembly:
  nav: modules/ROOT/nav.adoc
  root_level: 0
  insert_start_page: true
  section_merge_strategy: discrete
"#
    ))
}
