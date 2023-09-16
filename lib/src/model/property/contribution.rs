use super::{Error, Result};
use crate::model::identity::CompositionId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum Contribution {
    Composition,
    Translation(CompositionId),
    Modification(CompositionId),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde[try_from = "String"]]
#[serde[into = "String"]]
pub struct ContributionProperty(Contribution);

impl TryFrom<String> for ContributionProperty {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        let (contrib, reference) = match value.split_once("@") {
            Some((c, r)) => (c, Some(format!("@{r}"))),
            None => (value.as_str(), None),
        };

        match (contrib, reference) {
            ("composed", None) => Ok(ContributionProperty(Contribution::Composition)),
            ("translated", Some(id)) => Ok(ContributionProperty(Contribution::Translation(
                CompositionId::new(id),
            ))),
            ("modified", Some(id)) => Ok(ContributionProperty(Contribution::Modification(
                CompositionId::new(id),
            ))),
            _ => Err(Error::InvalidContribution(value)),
        }
    }
}

impl From<ContributionProperty> for String {
    fn from(value: ContributionProperty) -> Self {
        match value.0 {
            Contribution::Composition => "composed".to_owned(),
            Contribution::Translation(id) => format!("translated@compositions/{id}"),
            Contribution::Modification(id) => format!("modified@compositions/{id}"),
        }
    }
}

impl ContributionProperty {
    pub fn value(&self) -> Contribution {
        self.0.to_owned()
    }
}
