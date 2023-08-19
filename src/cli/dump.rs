use std::{env::current_dir, path::PathBuf};

use argh::FromArgs;

use crate::{
    database::{CollectionKey, Database},
    error::Result,
};

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "dump")]
/// Dump contents of the database
pub struct Config {
    /// the collection to dump from
    #[argh(positional)]
    collection: Option<String>,

    /// the database root directory
    #[argh(option)]
    root: Option<PathBuf>,
}

pub fn dump(args: Config) -> Result<()> {
    let root = args.root.unwrap_or(current_dir().unwrap_or_default());
    let db = Database::load(&root)?;

    println!("Database at {}", db.root().to_string_lossy().to_string());

    match args.collection {
        Some(c) => {
            let collection = CollectionKey::try_from(c)?;
            return dump_collection(&db, collection);
        }
        None => {
            for collection in CollectionKey::iter() {
                dump_collection(&db, *collection).expect("error while dumping");
            }
        }
    }

    Ok(())
}

fn dump_collection(db: &Database, collection: CollectionKey) -> Result<()> {
    let p = db.collection(collection);

    println!(
        "Dumping {} records from collection {:?}",
        p.len(),
        collection
    );
    for (id, data) in p {
        if let Ok(data) = serde_json::to_string(data.as_ref()) {
            println!("{}: {data}", id.value());
        }
    }

    Ok(())
}
