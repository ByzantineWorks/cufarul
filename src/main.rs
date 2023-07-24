mod fields;
mod error;
mod models;

use crate::models::{Person, Model};

fn main() {
    println!("Hello, world!");

    match Person::from_file(std::env::args().nth(1).unwrap_or(String::from("default.toml"))) {
        Ok(p) => println!("{:?}", p),
        Err(e) => println!("{}", e),
    }
}
