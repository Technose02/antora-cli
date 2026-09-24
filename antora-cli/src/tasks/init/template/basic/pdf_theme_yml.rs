use antora_fs::Resource;
use relative_path::RelativeFile;

pub(super) fn relative_file() -> RelativeFile {
    "pdf_theme.yml"
        .try_into()
        .expect("is a valid relative-path to a file")
}

pub(super) fn content() -> Resource {
    Resource::TextBased(
        r#"extends: default
image:
    border-color: #2f2f2f
    border-width: 1
    border-radius: 5
title-page: false
"#
        .to_string(),
    )
}
