mod fields;
mod error;
mod serde;
mod models;

use crate::models::{Person, Model};

fn main() {
    println!("Hello, world!");

    let p = Person::from_file(std::env::args().nth(1).unwrap_or(String::from("default.toml")));

    println!("{p:?}");
}
