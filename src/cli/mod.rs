use argh::FromArgs;

pub mod dump;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Command {
    Dump(dump::Config),
}

#[derive(FromArgs)]
/// Cufarul
pub struct Config {
    #[argh(subcommand)]
    pub command: Command,
}
