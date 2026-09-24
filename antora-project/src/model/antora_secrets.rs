use std::fmt::Display;

use crate::Error;
use serde::{Deserialize, Serialize};
use toml::{de::from_str as parse_toml, ser::to_string_pretty as write_toml};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AntoraSecrets {
    #[serde(rename = "access-tokens")]
    access_tokens: AccessTokens,
}

impl AntoraSecrets {
    pub fn git_credentials(&self) -> Option<String> {
        let gitlab_pat = &self.access_tokens.gitlab_pat;
        let gitlab_scheme = &self.access_tokens.gitlab_scheme;
        let gitlab_host = &self.access_tokens.gitlab_host;

        if gitlab_pat.is_empty() {
            None
        } else {
            Some(format!("{gitlab_scheme}://:{gitlab_pat}@{gitlab_host}"))
        }
    }

    pub fn confluence_pat(&self) -> Option<String> {
        if self.access_tokens.confluence_pat.is_empty() {
            None
        } else {
            Some(self.access_tokens.confluence_pat.clone())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokens {
    #[serde(rename = "confluence-pat")]
    confluence_pat: String,
    #[serde(rename = "gitlab-pat")]
    gitlab_pat: String,
    #[serde(rename = "gitlab-scheme")]
    gitlab_scheme: String,
    #[serde(rename = "gitlab-host")]
    gitlab_host: String,
}

impl Default for AccessTokens {
    fn default() -> Self {
        Self {
            confluence_pat: "<CONFLUENCE_PERSONAL_ACCESS_TOKEN>".to_owned(),
            gitlab_pat: "<GITLAB_ACCESS_TOKEN>".to_owned(),
            gitlab_scheme: "https".to_owned(),
            gitlab_host: "<GITLAB_HOST>".to_owned(),
        }
    }
}

impl TryFrom<&str> for AntoraSecrets {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_toml::<AntoraSecrets>(value).map_err(Error::TomlDeserializationError)
    }
}

impl Display for AntoraSecrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            write_toml(&self).expect("toml-serialization of AntoraSecrets must always work")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secrets_de_and_ser_work() {
        let mut antora_secrets: AntoraSecrets = AntoraSecrets::default();
        assert_eq!(
            antora_secrets.confluence_pat(),
            Some("<CONFLUENCE_PERSONAL_ACCESS_TOKEN>".to_owned())
        );
        assert_eq!(
            antora_secrets.git_credentials(),
            Some("https://:<GITLAB_ACCESS_TOKEN>@<GITLAB_HOST>".to_owned())
        );
        let mut serialized = antora_secrets.to_string();
        println!("{serialized}");

        assert!(serialized.contains("confluence-pat = \"<CONFLUENCE_PERSONAL_ACCESS_TOKEN>\""));
        assert!(serialized.contains("gitlab-pat = \"<GITLAB_ACCESS_TOKEN>\""));
        assert!(serialized.contains("gitlab-scheme = \"https\""));
        assert!(serialized.contains("gitlab-host = \"<GITLAB_HOST>\""));

        serialized = serialized.replace(
            "confluence-pat = \"<CONFLUENCE_PERSONAL_ACCESS_TOKEN>\"",
            "confluence-pat = \"01234\"",
        );
        serialized = serialized.replace(
            "gitlab-pat = \"<GITLAB_ACCESS_TOKEN>\"",
            "gitlab-pat = \"56789\"",
        );

        antora_secrets = AntoraSecrets::try_from(serialized.as_str()).unwrap();
        println!("{antora_secrets:#?}");

        assert!(antora_secrets.confluence_pat() == Some(String::from("01234")));
        assert!(
            antora_secrets.git_credentials() == Some(String::from("https://:56789@<GITLAB_HOST>"))
        );
    }
}
