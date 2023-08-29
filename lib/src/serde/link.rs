use super::{Error, NonEmptyString, Property, Result};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LinkProperty {
    content: NonEmptyString,
    url: NonEmptyString,
}

#[derive(Clone, Debug)]
pub enum ExternalLink {
    VideoUrl(Url),
    AudioUrl(Url),
    WikiUrl(Url),
    MiscUrl(Url, String),
}

impl TryFrom<LinkProperty> for ExternalLink {
    type Error = Error;
    fn try_from(value: LinkProperty) -> Result<Self> {
        let url = Url::parse(value.url.value()).map_err(|err| {
            Error::InvalidExternalLink(value.url.value().to_owned(), err.to_string())
        })?;

        Ok(match value.content.value().trim() {
            "video" => Self::VideoUrl(url),
            "audio" => Self::AudioUrl(url),
            "wiki" => Self::WikiUrl(url),
            other => Self::MiscUrl(url, other.to_owned()),
        })
    }
}

impl Property<ExternalLink> for LinkProperty {
    fn value(&self, _: Option<super::Lang>) -> Option<ExternalLink> {
        ExternalLink::try_from(self.to_owned()).ok()
    }
}
