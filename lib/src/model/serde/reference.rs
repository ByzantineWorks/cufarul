use crate::model::{entity::de::SerializeModel, CollectionKey};

#[derive(Debug)]
pub enum Reference<T>
where
    T: SerializeModel,
{
    Key(CollectionKey),
    Data(T),
    None,
}

impl<T> Default for Reference<T>
where
    T: SerializeModel,
{
    fn default() -> Self {
        Self::None
    }
}
