use std::process::exit;

use cli::{Config, Command};

mod cli;
mod database;
mod error;
mod fields;
mod models;


fn main() {
	let conf: Config = argh::from_env();
	let res = match conf.command {
		Command::Dump(c) => cli::dump::dump(c)
	};

	match res {
		Ok(_) => exit(0),
		Err(e) => {
			eprintln!("Error: {e}");
			exit(1);
		}
	}
}
