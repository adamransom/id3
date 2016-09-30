use std::io::{Read, BufReader};

use self::header::HeaderResult;
use self::header::HeaderBytes;

pub use self::error::Error;
pub use self::header::Header;
pub use self::header::HeaderFlags;
pub use self::version::Version;

mod error;
mod header;
mod version;

pub fn parse<R: Read>(reader: &mut BufReader<R>) -> HeaderResult<Header> {
    let mut bytes = [0u8; 10];
    try!(reader.read_exact(&mut bytes));

    if !is_valid(&bytes) {
        Err(Error::NotID3)
    } else {
        Header::new_with_bytes(&bytes)
    }
}

fn is_valid(bytes: &HeaderBytes) -> bool {
    let identifier = &bytes[0..3];

    identifier == b"ID3"
}
