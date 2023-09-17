use super::args::Args;
use argh::FromArgs;
use cufarul::{
    db::Database,
    error::Result,
    model::{CollectionKey, Composition, Lang, Model, Person, Publication, Taxonomy, Text},
    repo::{Cufarul, CufarulRepository, Repository, RepositorySpec},
};

/// dump contents of the database
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand, name = "dump")]
pub struct DumpConfig {
    /// id to dump
    #[argh(positional)]
    id: Option<String>,
}

fn dump_all(repo: CufarulRepository) -> Result<()> {
    for (id, node) in repo.db().nodes_iter() {
        let data = node.data();
        let object: &dyn Model = match id {
            CollectionKey::Person(_) => data
                .as_any()
                .downcast_ref::<Person>()
                .expect("internal error"),
            CollectionKey::Composition(_) => data
                .as_any()
                .downcast_ref::<Composition>()
                .expect("internal error"),
            CollectionKey::Publication(_) => data
                .as_any()
                .downcast_ref::<Publication>()
                .expect("internal error"),
            CollectionKey::Performance(_) => data
                .as_any()
                .downcast_ref::<Composition>()
                .expect("internal error"),
            CollectionKey::Text(_) => data
                .as_any()
                .downcast_ref::<Text>()
                .expect("internal error"),
            CollectionKey::Taxonomy(_) => data
                .as_any()
                .downcast_ref::<Taxonomy>()
                .expect("internal error"),
        };

        let out = serde_json::to_string(object).expect("error serializing");
        println!("{id}: {out}");
    }

    // println!("{:#?}", repo.db());
    // let author = CollectionKey::Person("ffddf9b9-1552-40a0-aefa-ef660479d329".to_owned());
    // println!("Showing all nodes authored by {author}:");

    // if let Some(edges) = repo.db().edges_to(author, ReferenceKey::Author) {
    //     for entry in edges {
    //         let object = repo
    //             .db()
    //             .get_node(entry.object().to_owned())
    //             .expect("oops")
    //             .data()
    //             .clone();
    //         let data: &dyn Model = object.as_any().downcast_ref::<Person>().expect("oops");
    //         println!(
    //             "{} authored {}: {}",
    //             entry.subject(),
    //             entry.object(),
    //             serde_json::to_string(data).expect("oops")
    //         );
    //     }
    // }

    Ok(())
}

pub fn dump(args: Args, config: DumpConfig) -> Result<()> {
    let mut repo = RepositorySpec::from_path(
        args.repo
            .unwrap_or(std::env::current_dir().unwrap_or_default())
            .as_path(),
    )
    .map_err(|err| err.into())
    .and_then(|spec| CufarulRepository::from_spec(spec))?;

    println!(
        "Found repository version {} at {:?}",
        repo.spec().version(),
        repo.spec().root()
    );

    println!("Syncing...");
    repo.sync()?;

    match config.id {
        Some(id) => {
            let node = repo.model_by_id(CollectionKey::Composition(id.into()), Some(Lang::RO))?;
            let repr = serde_json::to_string_pretty(&node).expect("ofof");

            println!("{repr}");
            Ok(())
        }
        None => dump_all(repo),
    }
}
