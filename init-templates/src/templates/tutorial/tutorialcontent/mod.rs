use antora_fs::Resource;
use relative_path::{Dirname, RelativeDir, create_relative_dir_from_dirname};

mod abbildung_svg;
mod antora_concepts_adoc;
mod antora_playbook_placeholder;
mod antora_yml_placeholder;
mod asciidoc_intro_adoc;
mod diagrams_adoc;
mod flowchart_mmd;
mod index_adoc;
mod sample_table;
mod sequence_puml;

pub(super) use abbildung_svg::content as abbildung_svg_content;
pub(super) use abbildung_svg::relative_file as abbildung_svg_relative_file;
pub(super) use antora_concepts_adoc::content as antora_concepts_adoc_content;
pub(super) use antora_concepts_adoc::relative_file as antora_concepts_adoc_relative_file;
pub(super) use antora_playbook_placeholder::content as antora_playbook_placeholder_content;
pub(super) use antora_playbook_placeholder::relative_file as antora_playbook_placeholder_relative_file;
pub(super) use antora_yml_placeholder::content as antora_yml_placeholder_content;
pub(super) use antora_yml_placeholder::relative_file as antora_yml_placeholder_relative_file;
pub(super) use asciidoc_intro_adoc::content as asciidoc_intro_adoc_content;
pub(super) use asciidoc_intro_adoc::relative_file as asciidoc_intro_adoc_relative_file;
pub(super) use diagrams_adoc::content as diagrams_adoc_content;
pub(super) use diagrams_adoc::relative_file as diagrams_adoc_relative_file;
pub(super) use flowchart_mmd::content as flowchart_mmd_content;
pub(super) use flowchart_mmd::relative_file as flowchart_mmd_relative_file;
pub(super) use index_adoc::content as index_adoc_content;
pub(super) use index_adoc::relative_file as index_adoc_relative_file;
pub(super) use sample_table::content as sample_table_content;
pub(super) use sample_table::relative_file as sample_table_relative_file;
pub(super) use sequence_puml::content as sequence_puml_content;
pub(super) use sequence_puml::relative_file as sequence_puml_relative_file;

fn placeholder_content() -> Resource {
    Resource::TextBased(String::from(
        "ICH BIN EIN PLATZHALTER FÜR DYNAMISCH EINGEBUNDENEN INHALT",
    ))
}

fn collected_dir() -> RelativeDir {
    create_relative_dir_from_dirname(
        Dirname::try_from("collected").expect("collected is a valid Dirname"),
    )
}
