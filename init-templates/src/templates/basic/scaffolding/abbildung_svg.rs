use antora_fs::Resource;
use relative_path::RelativeFile;

// scaffolding/images/abbildung.svg
pub fn relative_file() -> RelativeFile {
    "abbildung.svg"
        .try_into()
        .expect("antora.yml is a valid Filename")
}

pub fn content() -> Resource {
    Resource::TextBased(String::from(
        r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
    <circle cx="50" cy="50" r="40" stroke="black" stroke-width="2" fill="lightblue" />
</svg>"#,
    ))
}
