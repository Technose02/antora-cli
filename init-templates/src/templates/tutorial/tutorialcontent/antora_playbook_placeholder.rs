use antora_fs::Resource;
use relative_path::{Filename, RelativeFile};

// tutorial/examples/collected/antora-playbook.yml
pub fn resource_file() -> (RelativeFile, Resource) {
    (
        super::collected_dir().push_file(
            Filename::try_from("antora-playbook.yml")
                .expect("antora-playbook.yml is a valid Filename"),
        ),
        super::placeholder_content(),
    )
}
