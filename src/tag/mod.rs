//! Types, structs and functions related to reading an ID3v2 tag.

use std::io::Read;

use header::Header;
use frame::Frame;
use header;

pub use self::error::Error;

mod error;

/// A specialised `Result` type for tag reading operations.
pub type TagResult<T> = Result<T, Error>;

/// A type representing an ID3v2 tag.
///
/// # Reference
///
/// [ID3v2.3 Informal Standard](http://id3.org/id3v2.3.0)
#[derive(Debug, Default)]
pub struct Tag {
    header: Header,
    frame: Frame,
}

impl Tag {
    /// Construct a new tag from a reader.
    ///
    /// # Errors
    ///
    /// If there is an error reading the header, then this function will return
    /// `Error::Header`.
    pub fn from_reader<R: Read>(mut reader: &mut R) -> TagResult<Tag> {
        let header = try!(Header::from_reader(&mut reader.take(10)));
        let frame = try!(Frame::from_reader(&mut reader));

        let tag = Tag { header: header, frame: frame };

        Ok(tag)
    }

    /// Gets the header of the tag.
    pub fn header(&self) -> &Header {
        &self.header
    }
}
