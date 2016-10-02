use std::io::{Read, BufReader};

use self::header::HeaderResult;

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

    Header::new_with_bytes(&bytes)
}
