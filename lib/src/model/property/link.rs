use super::NonEmptyString;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LinkProperty {
    pub content: NonEmptyString,
    pub url: Url,
}

#[derive(Clone, Debug)]
pub enum ExternalLink {
    Video(Url),
    Audio(Url),
    Wiki(Url),
    Misc(Url, String),
}

impl From<LinkProperty> for ExternalLink {
    fn from(value: LinkProperty) -> Self {
        match value.content.value().trim() {
            "video" => Self::Video(value.url),
            "audio" => Self::Audio(value.url),
            "wiki" => Self::Wiki(value.url),
            other => Self::Misc(value.url, other.to_owned()),
        }
    }
}
