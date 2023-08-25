use crate::args::Args;
use cufarul::{
    model::{CollectionKey, Model, Person, ReferenceKey},
    repo::Repository,
};
use std::{process::exit, rc::Rc};

mod args;

fn main() {
    let args: Args = argh::from_env();
    let mut repo = cufarul::repo::RepositorySpec::from_path(
        args.repo
            .or(Some(std::env::current_dir().unwrap_or_default()))
            .unwrap()
            .as_path(),
    )
    .and_then(|spec| Repository::<CollectionKey, ReferenceKey>::try_from(spec))
    .unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        exit(1);
    });

    println!(
        "Found repository version {} at {:?}",
        repo.spec().version(),
        repo.spec().root()
    );

    let spanac_path = repo.spec().root().join("people").join("spanac.toml");
    let macaroana_path = repo.spec().root().join("people").join("macaroana.toml");

    let spanac = Person::load(spanac_path).expect("could not load spanac");
    let macaroana = Person::load(macaroana_path).expect("could not load macaroana");

    repo.db_mut()
        .insert_node(CollectionKey::Person("spanac".to_owned()), Rc::new(spanac))
        .expect("insert: spanac: something went wrong");
    repo.db_mut()
        .insert_node(
            CollectionKey::Person("macaroana".to_owned()),
            Rc::new(macaroana),
        )
        .expect("insert: macaroana: something went wrong");
    repo.db_mut()
        .insert_edge(
            CollectionKey::Person("spanac".to_owned()),
            CollectionKey::Person("macaroana".to_owned()),
            ReferenceKey::Author,
            None,
        )
        .expect("insert: edge: something went wrong");

    println!("{:#?}", repo.db());
}
