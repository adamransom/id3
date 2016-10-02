#[macro_use]
extern crate bitflags;

use std::io::{Read, BufReader};
use tag::Tag;
use header::Header;

pub mod tag;
pub mod header;
pub mod utils;

pub fn read<R: Read>(readable: R) {
    let mut reader = BufReader::new(readable);
    let tag = Tag::from_reader(&mut reader.take(3));

    match tag {
        Ok(tag) => println!("{:?}", tag),
        Err(err) => println!("Tag error: {}", err),
    }
}
