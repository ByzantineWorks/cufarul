use crate::args::Args;
use cufarul::{
    model::{EdgeKind, NodeKind},
    repo::Repository,
};
use std::process::exit;

mod args;

fn main() {
    let args: Args = argh::from_env();
    let mut repo = cufarul::repo::RepositorySpec::from_path(
        args.repo
            .or(Some(std::env::current_dir().unwrap_or_default()))
            .unwrap()
            .as_path(),
    )
    .and_then(|spec| Repository::<EdgeKind, NodeKind>::try_from(spec))
    .unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        exit(1);
    });

    println!(
        "Found repository version {} at {:?}",
        repo.spec().version(),
        repo.spec().root()
    );

    repo.db_mut()
        .insert_node("spanac".to_owned(), NodeKind::Person);
    repo.db_mut()
        .insert_node("macaroana".to_owned(), NodeKind::Person);
    repo.db_mut()
        .insert_edge(
            "spanac".to_owned(),
            "macaroana".to_owned(),
            EdgeKind::Author,
        )
        .expect("something went wrong");

    println!("{:?}", repo.db());
}
