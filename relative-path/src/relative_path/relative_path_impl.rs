use crate::{Dirname, Error, Filename};
use serde::{Deserialize, Serialize, de::Error as DeError};
use std::{fmt::Display, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File(pub(crate) Filename);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dir;

pub type RelativeDir = RelativePath<Dir>;
pub type RelativeFile = RelativePath<File>;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct RelativePath<T> {
    pub(crate) dirnames: Vec<Dirname>,
    pub(crate) kind: T,
}

impl Default for RelativeDir {
    fn default() -> Self {
        Self {
            dirnames: Vec::new(),
            kind: Dir,
        }
    }
}

pub fn new_relative_path() -> RelativePath<Dir> {
    RelativePath {
        dirnames: Vec::new(),
        kind: Dir,
    }
}

pub fn create_relative_dir_from_dirname(dirname: Dirname) -> RelativePath<Dir> {
    RelativePath::<Dir>::from(dirname)
}

pub fn create_relative_file_from_filename(filename: Filename) -> RelativePath<File> {
    RelativePath::<File>::from(filename)
}

impl From<Dirname> for RelativePath<Dir> {
    fn from(value: Dirname) -> Self {
        let dirnames = vec![value];
        Self {
            dirnames,
            kind: Dir,
        }
    }
}

impl From<Filename> for RelativePath<File> {
    fn from(value: Filename) -> Self {
        Self {
            dirnames: Vec::new(),
            kind: File(value),
        }
    }
}

impl RelativePath<Dir> {
    pub fn push_dir(&mut self, entry: Dirname) -> &mut Self {
        self.dirnames.push(entry);
        self
    }
    pub fn push_file(self, entry: Filename) -> RelativePath<File> {
        RelativePath {
            dirnames: self.dirnames,
            kind: File(entry),
        }
    }

    pub fn append<T>(mut self, mut other: RelativePath<T>) -> RelativePath<T> {
        self.dirnames.append(&mut other.dirnames);
        RelativePath {
            dirnames: self.dirnames,
            kind: other.kind,
        }
    }

    pub fn pop(&mut self) -> Option<Dirname> {
        self.dirnames.pop()
    }

    pub fn count(&self) -> usize {
        self.dirnames.len()
    }
}

impl RelativePath<File> {
    pub fn pop(self) -> RelativePath<Dir> {
        RelativePath {
            dirnames: self.dirnames,
            kind: Dir,
        }
    }

    pub fn count(&self) -> usize {
        self.dirnames.len() + 1
    }

    pub fn filename(&self) -> String {
        self.kind.0.0.clone()
    }
}

impl From<RelativePath<File>> for (RelativePath<Dir>, Filename) {
    fn from(value: RelativePath<File>) -> Self {
        let File(filename) = value.kind;

        let relative_path = RelativePath {
            dirnames: value.dirnames,
            kind: Dir,
        };
        (relative_path, filename)
    }
}

impl From<(RelativePath<Dir>, Filename)> for RelativePath<File> {
    fn from(value: (RelativePath<Dir>, Filename)) -> Self {
        Self {
            dirnames: value.0.dirnames,
            kind: File(value.1),
        }
    }
}

impl TryFrom<&str> for RelativePath<File> {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.replace("\\", "/");
        if !value.contains("/") {
            Ok(create_relative_file_from_filename(Filename::try_from(
                value.to_owned(),
            )?))
        } else {
            let mut split = value.split("/");
            let mut relative_dir = create_relative_dir_from_dirname(Dirname::try_from(
                split.next().unwrap().to_owned(),
            )?);
            let entries = split.map(str::to_owned).collect::<Vec<String>>();
            for dir in &entries[..entries.len() - 1] {
                relative_dir.push_dir(Dirname::try_from(dir.to_owned())?);
            }
            Ok(relative_dir.push_file(Filename::try_from(entries[entries.len() - 1].clone())?))
        }
    }
}

impl TryFrom<&String> for RelativePath<File> {
    type Error = Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<String> for RelativePath<File> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for RelativePath<Dir> {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.replace("\\", "/");
        if !value.contains("/") {
            Ok(create_relative_dir_from_dirname(Dirname::try_from(
                value.to_owned(),
            )?))
        } else {
            let mut split = value.split("/");
            let mut relative_dir = create_relative_dir_from_dirname(Dirname::try_from(
                split.next().unwrap().to_owned(),
            )?);
            for dir in split.map(str::to_owned) {
                relative_dir.push_dir(Dirname::try_from(dir)?);
            }
            Ok(relative_dir)
        }
    }
}

impl TryFrom<&String> for RelativePath<Dir> {
    type Error = Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<String> for RelativePath<Dir> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<RelativePath<Dir>> for PathBuf {
    fn from(value: RelativePath<Dir>) -> Self {
        let mut pb = PathBuf::with_capacity(value.dirnames.len());
        for Dirname(dirname) in value.dirnames.into_iter() {
            pb.push(dirname);
        }
        pb
    }
}

impl From<RelativePath<File>> for PathBuf {
    fn from(value: RelativePath<File>) -> Self {
        let (r, Filename(filename)) = value.into();
        let mut pb = Into::<PathBuf>::into(r);
        pb.push(filename);
        pb
    }
}

impl Display for RelativePath<Dir> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dirs = self.dirnames.iter();
        if let Some(Dirname(dir)) = dirs.next() {
            write!(f, "{dir}")?;
            for dir in dirs.map(|Dirname(dir)| dir) {
                write!(f, "/{dir}")?;
            }
            Ok(())
        } else {
            write!(f, "")
        }
    }
}

impl Display for RelativePath<File> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dirs = self.dirnames.iter();
        let File(Filename(filename)) = &self.kind;
        if let Some(Dirname(dir)) = dirs.next() {
            write!(f, "{dir}")?;
            for dir in dirs.map(|Dirname(dir)| dir) {
                write!(f, "/{dir}")?;
            }
            write!(f, "/")?;
        }
        write!(f, "{filename}")
    }
}

impl Serialize for RelativePath<Dir> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Serialize for RelativePath<File> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RelativePath<Dir> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        RelativeDir::try_from(value.as_str())
            .map_err(|e| DeError::custom(format!("error deserializing RelativeDir: {e}")))
    }
}

impl<'de> Deserialize<'de> for RelativePath<File> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        RelativeFile::try_from(value.as_str())
            .map_err(|e| DeError::custom(format!("error deserializing RelativeFile: {e}")))
    }
}
