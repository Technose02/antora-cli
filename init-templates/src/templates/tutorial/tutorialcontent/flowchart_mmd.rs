use antora_fs::Resource;
use relative_path::RelativeFile;

// tutorial/images/flowchart.mmd
pub fn resource_file() -> (RelativeFile, Resource) {
    (
        "flowchart.mmd"
            .try_into()
            .expect("flowchart.mmd is a valid filename"),
        Resource::TextBased(String::from(
            r#"graph TD
        A[Start] --> B{Entscheidung}
        B -->|Ja| C[Ergebnis 1]
        B -->|Nein| D[Ergebnis 2]
    "#,
        )),
    )
}
