use antora_fs::Resource;
use relative_path::RelativeFile;

// tutorial/partials/sample-table.adoc
pub fn resource_file() -> (RelativeFile, Resource) {
    (
        "sample-table.adoc"
            .try_into()
            .expect("sample-table.adoc is a valid filename"),
        Resource::TextBased(String::from(
            r#"| Name | Beschreibung

| Beispiel 1 | Beschreibung 1
| Beispiel 2 | Beschreibung 2
"#,
        )),
    )
}
