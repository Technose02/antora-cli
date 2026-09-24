use antora_fs::Resource;
use relative_path::{Filename, RelativeFile};

// scaffolding/examples/collected/antora-playbook.yml
pub fn relative_file() -> RelativeFile {
    super::collected_dir().push_file(
        Filename::try_from("antora-playbook.yml").expect("antora-playbook.yml is a valid Filename"),
    )
}

pub fn content() -> Resource {
    super::placeholder_content()
}
