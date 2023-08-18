use std::process::exit;

use cli::{Command, Config};

mod cli;
mod database;
mod error;
mod models;
mod serde;

fn main() {
    let conf: Config = argh::from_env();
    let res = match conf.command {
        Command::Dump(c) => cli::dump::dump(c),
    };

    match res {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }
}
