use antora_fs::Resource;
use relative_path::RelativeFile;

// tutorial/images/abbildung.svg
pub fn resource_file() -> (RelativeFile, Resource) {
    (
        "abbildung.svg"
            .try_into()
            .expect("antora.yml is a valid Filename"),
        Resource::TextBased(String::from(
            r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
    <circle cx="50" cy="50" r="40" stroke="black" stroke-width="2" fill="lightblue" />
</svg>"#,
        )),
    )
}
