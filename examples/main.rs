extern crate id3;

use std::error::Error;
use std::fs::File;
use std::path::Path;

use id3::tag::Tag;

fn main() {
    let path = Path::new("examples/test.mp3");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
    };

    let tag = Tag::from_reader(&mut file);

    match tag {
        Ok(tag) => println!("{:?}", tag),
        Err(err) => println!("{}", err),
    }
}
