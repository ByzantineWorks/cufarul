mod contribution;
mod generic;
mod link;
mod reference;
mod translation;

pub use self::contribution::ContributionProperty;
pub use self::generic::GenericProperty;
pub use self::link::LinkProperty;
pub use self::reference::ReferenceProperty;
pub use self::translation::TranslatableProperty;

use super::error::{Error, Result};
use super::serde::{Lang, NonEmptyString};

pub trait Property<T>
where
    T: Clone,
{
    fn value(&self, lang: Option<Lang>) -> Option<T>;
}
