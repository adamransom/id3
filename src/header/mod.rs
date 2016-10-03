//! Types, structs and functions related to reading an ID3v2 tag header.

use std::io::Read;

pub use self::error::Error;
pub use self::version::Version;

mod error;
mod version;

bitflags! {
    #[derive(Default)]
    flags HeaderFlags: u8 {
        const HEADER_UNSYNC = 0b10000000,
        const HEADER_EXTENDED = 0b01000000,
        const HEADER_EXPERIMENTAL = 0b00100000,
        const HEADER_FOOTER = 0b00010000,
    }
}

type HeaderBytes = [u8; 10];
/// A specialised `Result` type for header reading operations.
pub type HeaderResult<T> = Result<T, Error>;

/// A type representing the header of an ID3v2 tag.
///
/// # Reference
///
/// [ID3v2.3 Informal Standard (Section 3.1)](http://id3.org/id3v2.3.0#ID3v2_header)
#[derive(Debug, Default)]
pub struct Header {
    identifier: [u8; 3],
    version: Version,
    flags: HeaderFlags,
    size: u32,
}

impl Header {
    /// Construct a new header from a reader.
    ///
    /// # Guarantees
    ///
    /// This function will only ever read 10 bytes from the reader.
    ///
    /// # Errors
    ///
    /// If there is an error reading the bytes from the reader, then this function will return
    /// `Error::Io`.
    ///
    /// If the file identifier is incorrect (must be "ID3"), then this function will return
    /// `Error::InvalidIdentifier`.
    ///
    /// If there is an invalid version (neither versions can be 255), then this function will
    /// return `Error::InvalidVersion`.
    ///
    /// If there is an invalid size (it must be greater than 0 and less than 268435456), then this
    /// function will return `Error::InvalidSize`.
    ///
    /// If there is an unrecognized flag, then this function will return `Error::UnknownFlag`.
    pub fn from_reader<R: Read>(reader: &mut R) -> HeaderResult<Header> {
        let mut header: Self = Default::default();

        let bytes = try!(Header::read(reader));

        try!(header.set_identifier(&bytes));
        try!(header.set_version(&bytes));
        try!(header.set_size(&bytes));
        try!(header.set_flags(&bytes));

        Ok(header)
    }

    /// Gets the file identifier (currently always "ID3").
    pub fn identifier(&self) -> &[u8; 3] {
        &self.identifier
    }

    /// Gets the version of the ID3v2 tag.
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Gets the size of the tag (not including the header or footer). This will always be
    /// greater than 0 and less than 268435456.
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Gets whether or not unsynchronization is used.
    pub fn is_unsynchronized(&self) -> bool {
        self.flags.intersects(HEADER_UNSYNC)
    }

    /// Gets whether or not the header is followed by an extended header.
    pub fn has_extended_header(&self) -> bool {
        self.flags.intersects(HEADER_EXTENDED)
    }

    /// Gets whether or not the tag is in an experimental stage.
    pub fn is_experimental(&self) -> bool {
        self.flags.intersects(HEADER_EXPERIMENTAL)
    }

    /// Gets whether or not the tag has a footer.
    ///
    /// Since this library only supports `v2.3.0`, a `v2.4.0` footer will never be parsed, but we
    /// need to know one exists so it can be safely skipped.
    pub fn has_footer(&self) -> bool {
        self.flags.intersects(HEADER_FOOTER)
    }

    /// Read and return 10 bytes from the reader.
    fn read<R: Read>(reader: &mut R) -> HeaderResult<HeaderBytes> {
        let mut bytes = [0u8; 10];
        try!(reader.read_exact(&mut bytes));

        Ok(bytes)
    }

    /// Set the file identifier (currently always "ID3").
    fn set_identifier(&mut self, bytes: &HeaderBytes) -> HeaderResult<()> {
        self.identifier = [bytes[0], bytes[1], bytes[2]];

        if &self.identifier != b"ID3" {
            Err(Error::InvalidIdentifier)
        } else {
            Ok(())
        }
    }

    /// Set the major and revision versions.
    fn set_version(&mut self, bytes: &HeaderBytes) -> HeaderResult<()> {
        self.version.major = bytes[3];
        self.version.revision = bytes[4];

        if self.version.major < 0xFF && self.version.revision < 0xFF {
            Ok(())
        } else {
            Err(Error::InvalidVersion)
        }
    }

    /// Set the header size (not including header, or footer if present).
    fn set_size(&mut self, bytes: &HeaderBytes) -> HeaderResult<()> {
        use utils;

        self.size = utils::synchsafe_to_u32(&bytes[6..10]).unwrap_or(0);

        if self.size > 0 && self.size < 0x1000_0000 {
            Ok(())
        } else {
            Err(Error::InvalidSize)
        }
    }

    /// Set the flags of the tag.
    fn set_flags(&mut self, bytes: &HeaderBytes) -> HeaderResult<()> {
        self.flags = match HeaderFlags::from_bits(bytes[5]) {
            Some(flags) => flags,
            None => return Err(Error::UnknownFlag),
        };

        Ok(())
    }
}
