mod datatypes;
mod fields;
mod lang;
mod translation;

pub use datatypes::NonEmptyString;
pub use datatypes::VersionInfo;
pub use fields::GenericField;
pub use fields::TranslatableField;
pub use lang::Lang;

/* Testing */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn test_non_empty_string() {
        assert!(NonEmptyString::try_from(String::from("")).is_err());
    }

    #[test]
    fn test_lang_string_serde() -> Result<()> {
        assert_eq!(String::from(Lang::AR), String::from("ar"));
        assert_eq!(String::from(Lang::EN), String::from("en"));
        assert_eq!(String::from(Lang::GR), String::from("gr"));
        assert_eq!(String::from(Lang::RO), String::from("ro"));

        assert_eq!(Lang::AR, Lang::try_from(String::from("ar"))?);
        assert_eq!(Lang::EN, Lang::try_from(String::from("en"))?);
        assert_eq!(Lang::GR, Lang::try_from(String::from("gr"))?);
        assert_eq!(Lang::RO, Lang::try_from(String::from("ro"))?);

        assert!(Lang::try_from(String::from("unknown")).is_err());

        Ok(())
    }
}
