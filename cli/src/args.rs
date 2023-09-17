use argh::FromArgs;
use std::path::PathBuf;

use crate::dump::DumpConfig;

#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Dump(DumpConfig),
}

/// cufarul-cli
#[derive(Clone, FromArgs)]
pub struct Args {
    /// path to the repository root, defaults to the current working directory
    #[argh(option)]
    pub repo: Option<PathBuf>,

    /// the subcommand to execute
    #[argh(subcommand)]
    pub command: Command,
}
