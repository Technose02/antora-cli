use relative_path::RelativeFile;

pub enum Resource {
    TextBased(String),
    Binary(Vec<u8>),
}

pub struct ResourceFile {
    pub(crate) path: RelativeFile,
    pub(crate) content: Resource,
}

impl From<(RelativeFile, Resource)> for ResourceFile {
    fn from(value: (RelativeFile, Resource)) -> Self {
        ResourceFile {
            path: value.0,
            content: value.1,
        }
    }
}
