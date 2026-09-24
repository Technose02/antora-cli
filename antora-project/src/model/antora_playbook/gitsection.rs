use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct GitCredentials {
    pub path: String,
}

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
pub struct GitSection {
    pub credentials: GitCredentials,
}

// region:      --- Builder(s)

#[derive(Default)]
pub struct GitCredentialsBuilder {
    value: GitCredentials,
}

#[expect(unused)]
impl GitCredentialsBuilder {
    pub fn new() -> Self {
        GitCredentialsBuilder {
            value: GitCredentials::default(),
        }
    }

    pub fn init(git_credentials: GitCredentials) -> Self {
        GitCredentialsBuilder {
            value: git_credentials,
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.value.path = path.into();
        self
    }

    pub fn build(self) -> GitCredentials {
        self.value
    }
}

#[derive(Default)]
pub struct GitSectionBuilder {
    value: GitSection,
}

#[expect(unused)]
impl GitSectionBuilder {
    pub fn new() -> Self {
        GitSectionBuilder {
            value: GitSection::default(),
        }
    }

    pub fn init(git_section: GitSection) -> Self {
        GitSectionBuilder { value: git_section }
    }

    pub fn credentials(mut self, credentials: impl Into<GitCredentials>) -> Self {
        self.value.credentials = credentials.into();
        self
    }

    pub fn build(self) -> GitSection {
        self.value
    }
}

// endregion:   --- Builder(s)
