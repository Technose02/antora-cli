use antora_fs::Resource;
use relative_path::RelativeFile;

// scaffolding/images/flowchart.mmd
pub fn relative_file() -> RelativeFile {
    "flowchart.mmd"
        .try_into()
        .expect("flowchart.mmd is a valid filename")
}

pub fn content() -> Resource {
    Resource::TextBased(String::from(
        r#"graph TD
        A[Start] --> B{Entscheidung}
        B -->|Ja| C[Ergebnis 1]
        B -->|Nein| D[Ergebnis 2]
    "#,
    ))
}
