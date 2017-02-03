use std::io::prelude::*;
use std::fs::File;

mod regular;
use regular::Regular;

#[macro_use]
extern crate regular_derive;

#[derive(Regular)]
struct Person {
    name: String,
}

#[derive(Regular)]
struct Dog {
    name: String,
}

fn main() {
    let max = Person { name: String::from("Maxwell") };
    max.save();

    let lola = Dog { name: String::from("Lola") };
    lola.save();
}
