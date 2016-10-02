#[macro_use]
extern crate bitflags;

use std::io::{Read, BufReader};
use header::Header;

pub mod header;
pub mod utils;

pub fn read<R: Read>(readable: R) {
    let reader = BufReader::new(readable);
    let header = Header::from_reader(&mut reader.take(10));

    match header {
        Ok(header) => println!("{:?}", header),
        Err(err) => println!("Error parsing header ({})", err),
    }
}
