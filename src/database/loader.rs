use super::{spec::DatabaseSpec, Database};

#[derive(Debug)]
pub struct LoadPath {
	pub collection: String,
	pub id: String,
}

pub type LoadSpec = Vec<LoadPath>;

impl TryFrom<DatabaseSpec> for LoadSpec {
	type Error = crate::error::Error;
	fn try_from(spec: DatabaseSpec) -> Result<Self, Self::Error> {
		let mut res = LoadSpec::new();
		for collection in spec.collections() {
			let collection_dir = spec.root().join(collection);
			for entry in std::fs::read_dir(collection_dir)? {
				let path = entry?.path();
				if path.is_file() && path.extension().unwrap_or_default() == "toml" {
					res.push(LoadPath {
						collection: collection.clone(),
						id: path.file_stem().unwrap_or_default().to_string_lossy().to_string()
					});
				}
			}
		}

		Ok(res)
	}
}

pub fn from_file(db_spec: &DatabaseSpec, load_spec: Option<LoadSpec>) -> Database {
	let paths = load_spec.unwrap_or(LoadSpec::try_from(db_spec.clone()).unwrap_or_default());


	Ok(())
}
