//! Types, structs and functions related to reading the header of a frame in an ID3v2 tag.

use std::io::Read;
use std::result;

pub use self::error::Error;

mod error;

bitflags! {
    #[derive(Default)]
    flags StatusFlags: u8 {
        const STATUS_TAG_PRESERVE = 0b1000_0000,
        const STATUS_FILE_PRESERVE = 0b0100_0000,
        const STATUS_READ_ONLY = 0b0010_0000,
    }
}

bitflags! {
    #[derive(Default)]
    flags EncodingFlags: u8 {
        const ENCODING_COMPRESSION = 0b1000_0000,
        const ENCODING_ENCRYPTION = 0b0100_0000,
        const ENCODING_GROUPING = 0b0010_0000,
    }
}

type HeaderBytes = [u8; 10];
/// A specialised `Result` type for frame header reading operations.
pub type Result<T> = result::Result<T, Error>;

/// A type representing the header of a frame in an ID3v2 tag.
///
/// # Reference
///
/// [ID3v2.3 Informal Standard (Section 3.3)](http://id3.org/id3v2.3.0#ID3v2_frame_overview)
#[derive(Debug, Default)]
pub struct Header {
    frame_id: [u8; 4],
    size: u32,
    status_flags: StatusFlags,
    encoding_flags: EncodingFlags,
}

impl Header {
    /// Construct a new frame header from a reader.
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
    /// If there is an invalid size (it must be greater than 0), then this function will 
    /// return `Error::InvalidSize`.
    ///
    /// If there is an unrecognized flag, then this function will return `Error::UnknownFlag`.
    pub fn from_reader<R: Read>(reader: &mut R, version: u8) -> Result<Header> {
        let mut header: Self = Default::default();

        let bytes = try!(Header::read(reader));

        try!(header.set_frame_id(&bytes));
        try!(header.set_size(&bytes, version));
        try!(header.set_status_flags(&bytes));
        try!(header.set_encoding_flags(&bytes));

        Ok(header)
    }

    /// Gets the frame ID (made out of the characters capital A-Z and 0-9).
    pub fn frame_id(&self) -> &[u8; 4] {
        &self.frame_id
    }

    /// Gets the size of the frame (not including the header). This will always be greater than 0.
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Gets whether or not to preserve the frame if it is unknown and the tag is altered in any
    /// way. This applies to all kinds of alterations, including adding more padding and reordering the frames.
    pub fn should_preserve_tag(&self) -> bool {
        self.status_flags.intersects(STATUS_TAG_PRESERVE)
    }

    /// Gets whether or not to preserve the frame if it is unknown and the file, excluding the tag, is altered.
    pub fn should_preserve_file(&self) -> bool {
        self.status_flags.intersects(STATUS_FILE_PRESERVE)
    }

    /// Gets whether or not the contents of the frame is intended to be read only. Changing the contents might
    /// break something, e.g. a signature.
    pub fn is_read_only(&self) -> bool {
        self.status_flags.intersects(STATUS_READ_ONLY)
    }

    /// Gets whether or not the frame is compressed.
    pub fn is_compressed(&self) -> bool {
        self.encoding_flags.intersects(ENCODING_COMPRESSION)
    }

    /// Gets whether or not the frame is enrypted.
    pub fn is_encrypted(&self) -> bool {
        self.encoding_flags.intersects(ENCODING_ENCRYPTION)
    }

    /// Gest whether or not this frame belongs in a group with other frames.
    pub fn is_grouped(&self) -> bool {
        self.encoding_flags.intersects(ENCODING_GROUPING)
    }

    /// Read and return 10 bytes from the reader.
    fn read<R: Read>(reader: &mut R) -> Result<HeaderBytes> {
        let mut bytes = [0u8; 10];
        try!(reader.read_exact(&mut bytes));

        Ok(bytes)
    }

    /// Set the frame ID.
    fn set_frame_id(&mut self, bytes: &HeaderBytes) -> Result<()> {
        self.frame_id = [bytes[0], bytes[1], bytes[2], bytes[3]];

        Ok(())
    }

    /// Set the frame size (not including the header).
    fn set_size(&mut self, bytes: &HeaderBytes, version: u8) -> Result<()> {
        use utils;

        self.size = match version {
            3 => {
                (bytes[4] as u32) << 24 |
                (bytes[5] as u32) << 16 |
                (bytes[6] as u32) << 8  |
                (bytes[7] as u32)
            },
            4 => utils::synchsafe_to_u32(&bytes[4..8]).unwrap_or(0),
            _ => 0,
        };

        if self.size > 0 {
            Ok(())
        } else {
            Err(Error::InvalidSize)
        }
    }

    /// Set the status flags of the frame.
    fn set_status_flags(&mut self, bytes: &HeaderBytes) -> Result<()> {
        self.status_flags = match StatusFlags::from_bits(bytes[8]) {
            Some(flags) => flags,
            None => return Err(Error::UnknownFlag),
        };

        Ok(())
    }

    /// Set the encoding flags of the frame.
    fn set_encoding_flags(&mut self, bytes: &HeaderBytes) -> Result<()> {
        self.encoding_flags = match EncodingFlags::from_bits(bytes[9]) {
            Some(flags) => flags,
            None => return Err(Error::UnknownFlag),
        };

        Ok(())
    }
}
