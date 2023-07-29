use std::{path::{Path, PathBuf}, collections::BTreeMap};

use crate::{error::{Result, Error}, fields::NonEmptyString, models::{Person, Model}};

type Collection<T> = BTreeMap<NonEmptyString, T>;

pub struct Database {
	root: PathBuf,
	people: Collection<Person>,
}

impl Database {
	pub fn load(root: &Path) -> Result<Self> {
		let mut current = std::fs::canonicalize(root)?;

		loop {
			if current.join(".cufarul").exists() {
				break;
			}

			current = match current.parent() {
				Some(p) => p.canonicalize()?,
				None => return Err(Error::NoDatabase),
			};
		}

		let mut people: Collection<Person> = Collection::new();
		let people_entries = std::fs::read_dir(current.join("people"))?;
		for entry in people_entries {
			if let Ok(entry) = entry {
				let path = entry.path();
				if path.is_file() && path.extension().unwrap_or_default() == "toml" {
					let tmp = path.clone();
					let p: Person = Person::from_file(path)?;
					let stem = tmp.file_stem().unwrap().to_string_lossy().to_string();
					let id: NonEmptyString = NonEmptyString::try_from(stem)?;
					people.insert(id, p);
				}
			}
		}

		Ok(Database {
			root: current.to_owned(),
			people: people,
		})
	}

	pub fn root(&self) -> &PathBuf {
		&self.root
	}

	pub fn people(&self) -> &Collection<Person> {
		&self.people
	}
}
