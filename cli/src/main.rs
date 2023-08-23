use crate::args::Args;
use cufarul::repo::Repository;
use std::process::exit;

mod args;

fn main() {
    let args: Args = argh::from_env();
    let repo = cufarul::repo::RepositorySpec::from_path(
        args.repo
            .or(Some(std::env::current_dir().unwrap_or_default()))
            .unwrap()
            .as_path(),
    )
    .and_then(|spec| Repository::try_from(spec))
    .unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        exit(1);
    });

    println!(
        "Found repository version {} at {:?}",
        repo.spec().version(),
        repo.spec().root()
    );
}
