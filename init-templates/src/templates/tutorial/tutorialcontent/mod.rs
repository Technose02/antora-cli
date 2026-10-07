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

pub(super) use abbildung_svg::resource_file as abbildung_svg;
pub(super) use antora_concepts_adoc::resource_file as antora_concepts_adoc;
pub(super) use antora_playbook_placeholder::resource_file as antora_playbook_placeholder;
pub(super) use antora_yml_placeholder::resource_file as antora_yml_placeholder;
pub(super) use asciidoc_intro_adoc::resource_file as asciidoc_intro_adoc;
pub(super) use diagrams_adoc::resource_file as diagrams_adoc;
pub(super) use flowchart_mmd::resource_file as flowchart_mmd;
pub(super) use index_adoc::resource_file as index_adoc;
pub(super) use sample_table::resource_file as sample_table;

pub(super) use sequence_puml::resource_file as sequence_puml;

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
