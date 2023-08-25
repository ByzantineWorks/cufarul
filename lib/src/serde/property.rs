use super::Lang;

pub trait Property<T>
where
    T: Clone,
{
    fn value(&self, lang: Option<Lang>) -> Option<T>;
}
