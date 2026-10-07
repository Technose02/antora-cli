use relative_path::RelativeFile;

#[derive(Clone, Debug)]
pub enum Resource {
    TextBased(String),
    Binary(Vec<u8>),
}

impl From<String> for Resource {
    fn from(value: String) -> Self {
        Resource::TextBased(value)
    }
}

impl From<Vec<u8>> for Resource {
    fn from(value: Vec<u8>) -> Self {
        Resource::Binary(value)
    }
}

pub trait ResourceFile {
    fn path(&self) -> RelativeFile;
    fn content(&self) -> Resource;
    fn split(self) -> (RelativeFile, Resource);
}

impl ResourceFile for (RelativeFile, Resource) {
    fn path(&self) -> RelativeFile {
        self.0.clone()
    }
    fn content(&self) -> Resource {
        self.1.clone()
    }
    fn split(self) -> (RelativeFile, Resource) {
        (self.0, self.1)
    }
}

impl<C: FnOnce() -> Resource> From<C> for Resource {
    fn from(value: C) -> Self {
        value()
    }
}

//#[derive(Clone, Debug)]
//pub struct ResourceFile {
//    pub(crate) path: RelativeFile,
//    pub(crate) content: Resource,
//}

//impl From<(RelativeFile, Resource)> for ResourceFile {
//    fn from(value: (RelativeFile, Resource)) -> Self {
//        ResourceFile {
//            path: value.0,
//            content: value.1,
//        }
//    }
//}

//pub trait IntoResourceFile {
//    fn into(self) -> ResourceFile;
//}

//impl<P, C, R> IntoResourceFile for (P, C)
//where
//    P: FnOnce() -> RelativeFile,
//    C: FnOnce() -> R,
//    Resource: From<R>,
//{
//    fn into(self) -> ResourceFile {
//        ResourceFile {
//            path: self.0(),
//            content: Resource::from((self.1)()),
//        }
//    }
//}

//impl<I> From<I> for ResourceFile
//where
//    I: IntoResourceFile,
//{
//    fn from(value: I) -> Self {
//        value.into()
//    }
//}
