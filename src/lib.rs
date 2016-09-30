use std::io::{Read, BufReader};

pub mod header;
pub mod utils;

pub fn read<R: Read>(readable: R) {
    let mut reader = BufReader::new(readable);
    let header = header::parse(&mut reader);

    match header {
        Ok(header) => println!("{:?}", header),
        Err(err) => println!("Error parsing header ({})", err),
    }
}
