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
    let collection = args.collection.unwrap_or_default().to_string();
    let root = args.root.unwrap_or(current_dir().unwrap_or_default());

    let db = Database::load(&root)?;

    println!("Database at {}", db.root().to_string_lossy().to_string());
    println!("Collection: {}", collection);

    match collection.as_str() {
        "people" => {
            let p = db.collection(CollectionKey::People);

            println!("Dumping {} records", p.len());
            for (id, data) in p {
                if let Ok(data) = serde_json::to_string(data.as_ref()) {
                    println!("{}: {data}", id.value());
                }
            }
        }
        _ => {
            println!("not supported yet");
        }
    }

    Ok(())
}
