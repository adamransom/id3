use std::io::Read;

use header::Header;

pub use self::error::Error;

mod error;

pub type TagResult<T> = Result<T, Error>;

#[derive(Debug, Default)]
pub struct Tag {
    pub header: Header,
}

impl Tag {
    pub fn from_reader<R: Read>(reader: &mut R) -> TagResult<Tag> {
        let tag = Tag {
            header: try!(Header::from_reader(&mut reader.take(10))),
        };

        Ok(tag)
    }
}
