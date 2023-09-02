use crate::args::Args;
use cufarul::{
    db::Database,
    error::Result,
    model::{CollectionKey, Composition, Model, Person},
    repo::{CufarulRepository, Repository, RepositorySpec},
};

mod args;

fn main() -> Result<()> {
    let args: Args = argh::from_env();
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

    for (id, node) in repo.db().nodes_iter() {
        let object: &dyn Model = match id {
            CollectionKey::Person(_) => node
                .as_any()
                .downcast_ref::<Person>()
                .expect("internal error"),
            CollectionKey::Composition(_) => node
                .as_any()
                .downcast_ref::<Composition>()
                .expect("internal error"),
        };

        let out = serde_json::to_string(object).expect("error serializing");
        println!("{id}: {out}");
    }

    println!("{:#?}", repo.db());
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
