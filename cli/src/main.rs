use crate::args::Args;
use cufarul::{
    error::Result,
    model::{CollectionKey, Model, Person, ReferenceKey},
    repo::{Repository, RepositorySpec},
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
    .and_then(|spec| Repository::try_from(spec))?;

    println!(
        "Found repository version {} at {:?}",
        repo.spec().version(),
        repo.spec().root()
    );

    println!("Syncing...");
    repo.sync()?;

    for (id, node) in repo.db().nodes() {
        let data = node.data();
        let object: &dyn Model = match id {
            CollectionKey::Person(_) => data
                .as_any()
                .downcast_ref::<Person>()
                .expect("internal error"),
        };

        let out = serde_json::to_string(object).expect("error serializing");
        println!("{id}: {out}");
    }

    let author = CollectionKey::Person("macaroana".to_owned());
    println!("Showing all nodes authored by {author}:");

    if let Some(edges) = repo.db().edges_to(author, ReferenceKey::Author) {
        for entry in edges {
            let object = repo
                .db()
                .get_node(entry.object().to_owned())
                .expect("oops")
                .data()
                .clone();
            let data: &dyn Model = object.as_any().downcast_ref::<Person>().expect("oops");
            println!(
                "{} authored {}: {}",
                entry.subject(),
                entry.object(),
                serde_json::to_string(data).expect("oops")
            );
        }
    }

    Ok(())
}
