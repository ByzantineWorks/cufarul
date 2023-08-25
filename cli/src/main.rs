use crate::args::Args;
use cufarul::{
    model::{EdgeKind, NodeKind, Person},
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
    .and_then(|spec| Repository::<NodeKind, EdgeKind>::try_from(spec))
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
        .insert_node(NodeKind::Person("spanac".to_owned()), Rc::new(Person {}))
        .expect("insert: spanac: something went wrong");
    repo.db_mut()
        .insert_node(NodeKind::Person("macaroana".to_owned()), Rc::new(Person {}))
        .expect("insert: macaroana: something went wrong");
    repo.db_mut()
        .insert_edge(
            NodeKind::Person("spanac".to_owned()),
            NodeKind::Person("macaroana".to_owned()),
            EdgeKind::Author,
            None,
        )
        .expect("insert: edge: something went wrong");

    println!("{:#?}", repo.db());
}
