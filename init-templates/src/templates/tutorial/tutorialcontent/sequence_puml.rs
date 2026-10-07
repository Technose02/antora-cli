use antora_fs::Resource;
use relative_path::RelativeFile;

// tutorial/images/sequence.puml
pub fn resource_file() -> (RelativeFile, Resource) {
    (
        "sequence.puml"
            .try_into()
            .expect("sequence.puml is a valid filename"),
        Resource::TextBased(String::from(
            r#"@startuml
Alice -> Bob: Hallo
Bob --> Alice: Hallo zurück
@enduml"#,
        )),
    )
}
