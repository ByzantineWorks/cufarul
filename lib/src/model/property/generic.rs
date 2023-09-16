use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct GenericProperty<T> {
    #[serde(flatten)]
    data: T,
}

impl<T> GenericProperty<T>
where
    T: Clone,
{
    pub fn value(&self) -> T {
        self.data.to_owned()
    }
}
