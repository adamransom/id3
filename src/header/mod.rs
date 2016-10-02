use std::io::Read;

pub use self::error::Error;
pub use self::version::Version;

mod error;
mod version;

bitflags! {
    /// ID3v2 header flags.
    ///
    /// This actually includes one v4.0 flag (`HEADER_FOOTER`) so we are still able to handle v4.0
    /// tags and safely skip over the footer, without actually parsing it.
    ///
    /// # Flags
    ///
    /// - `HEADER_UNSYNC` - Indicates whether or not unsynchronisation is used.
    /// - `HEADER_EXTENDED` - Indicates whether or not the header is followed by an extended header.
    /// - `HEADER_EXPERIMENTAL` - Indicates whether the tag is in an experimental stage.
    /// - `HEADER_FOOTER` - Indicates that a footer is present at the very end of the tag.
    #[derive(Default)]
    pub flags HeaderFlags: u8 {
        const HEADER_UNSYNC = 0b10000000,
        const HEADER_EXTENDED = 0b01000000,
        const HEADER_EXPERIMENTAL = 0b00100000,
        const HEADER_FOOTER = 0b00010000,
    }
}

pub type HeaderBytes = [u8; 10];
pub type HeaderResult<T> = Result<T, Error>;

/// A type representing the header of an ID3v2 tag.
///
/// # Reference
///
/// [ID3 tag version 2.3.0](http://id3.org/id3v2.3.0#ID3v2_header)
#[derive(Debug, Default)]
pub struct Header {
    /// The file identifier (currently always "ID3").
    pub identifier: [u8; 3],
    /// The version of the ID3v2 tag.
    pub version: Version,
    /// The flags set for the tag.
    pub flags: HeaderFlags,
    /// The size of the tag (not including the header or footer). This must be greater than 0 and
    /// less than 268435456.
    pub size: u32,
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

    /// Read and return 10 bytes from the reader.
    fn read<R: Read>(reader: &mut R) -> HeaderResult<[u8; 10]> {
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
