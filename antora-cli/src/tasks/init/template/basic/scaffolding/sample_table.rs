use antora_fs::Resource;
use relative_path::RelativeFile;

// scaffolding/partials/sample-table.adoc
pub fn relative_file() -> RelativeFile {
    "sample-table.adoc"
        .try_into()
        .expect("sample-table.adoc is a valid filename")
}

pub fn content() -> Resource {
    Resource::TextBased(String::from(
        r#"| Name | Beschreibung

| Beispiel 1 | Beschreibung 1
| Beispiel 2 | Beschreibung 2
"#,
    ))
}
