use crate::args::Args;
use args::Command;
use cufarul::error::Result;
use dump::dump;
use std::process::exit;

mod args;
mod dump;

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let res = match args.command {
        Command::Dump(_) => dump(args),
    };

    match res {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}
