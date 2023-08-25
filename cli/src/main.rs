use crate::args::Args;
use cufarul::{error::Result, repo::Repository};

mod args;

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let mut repo = cufarul::repo::RepositorySpec::from_path(
        args.repo
            .or(Some(std::env::current_dir().unwrap_or_default()))
            .unwrap()
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

    println!("{:#?}", repo.db());
    Ok(())
}
