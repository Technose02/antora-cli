use antora_fs::Resource;
use relative_path::RelativeFile;

// scaffolding/images/sequence.puml
pub fn relative_file() -> RelativeFile {
    "sequence.puml"
        .try_into()
        .expect("sequence.puml is a valid filename")
}

pub fn content() -> Resource {
    Resource::TextBased(String::from(
        r#"@startuml
Alice -> Bob: Hallo
Bob --> Alice: Hallo zurück
@enduml"#,
    ))
}
