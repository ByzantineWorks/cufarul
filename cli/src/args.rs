use std::path::PathBuf;

use argh::FromArgs;

/// cufarul-cli
#[derive(FromArgs)]
pub struct Args {
    /// path to the repository root, defaults to the current working directory
    #[argh(option)]
    pub repo: Option<PathBuf>,
}
