use super::property::Property;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GenericProperty<T> {
    #[serde(flatten)]
    data: T,
}

impl<T> Property<T> for GenericProperty<T>
where
    T: Clone,
{
    fn value(&self, _: Option<super::Lang>) -> Option<T> {
        Some(self.data.clone())
    }
}
