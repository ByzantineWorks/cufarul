#[derive(Debug)]
pub enum Error {
	ValueNotFound,
	LanguageNotSupported,
	RuntimeError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let error_msg: String = match self {
			Self::ValueNotFound => String::from("no value found"),
			Self::LanguageNotSupported => String::from("language code not supported"),
			Self::RuntimeError(e) => e.to_string(),
		};
		write!(f, "Error: {error_msg}")
	}
}

impl std::error::Error for Error {}
