extern crate id3;

use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("examples/test.mp3");
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
    };

    id3::read(file);
}
