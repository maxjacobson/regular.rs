use std::io::prelude::*;
use std::fs::File;

struct Person {
    name: String,
}

trait Regular {
    fn save(&self);
    fn to_s(&self) -> String;
}

impl Regular for Person {
    fn save(&self) {
        let path = "./persons.txt";
        let mut f = File::create(path).unwrap();
        f.write(self.to_s().as_bytes()).unwrap();
    }

    fn to_s(&self) -> String {
        format!("{}", self.name)
    }
}

fn main() {
    let max = Person { name: String::from("Maxwell") };

    max.save();
}
