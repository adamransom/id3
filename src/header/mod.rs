use std::io::{Read, BufReader};

mod error;

pub use self::error::HeaderError;

bitflags! {
    #[derive(Default)]
    pub flags HeaderFlags: u8 {
        const HEADER_UNSYNC = 0b10000000,
        const HEADER_EXTENDED = 0b01000000,
        const HEADER_EXPERIMENTAL = 0b00100000,
        const HEADER_FOOTER = 0b00010000,
    }
}

#[derive(Debug, Default)]
pub struct Version {
    pub major: u8,
    pub revision: u8,
}

#[derive(Debug, Default)]
pub struct Header {
    pub version: Version,
    pub flags: HeaderFlags,
    pub size: u32,
}

type HeaderBytes = [u8; 10];
type HeaderResult<T> = Result<T, HeaderError>;

impl Header {
    fn new_with_bytes(bytes: &HeaderBytes) -> HeaderResult<Header> {
        let mut header: Self = Default::default();

        header.set_version(bytes);
        try!(header.set_size(bytes));
        try!(header.set_flags(bytes));

        Ok(header)
    }

    fn set_version(&mut self, bytes: &HeaderBytes) {
        self.version.major = bytes[3];
        self.version.revision = bytes[4];
    }

    fn set_size(&mut self, bytes: &HeaderBytes) -> HeaderResult<()> {
        use utils;

        self.size = utils::synchsafe_to_u32(&bytes[6..10]).unwrap();

        if self.size > 0 {
            Ok(())
        } else {
            Err(HeaderError::InvalidSize)
        }
    }

    fn set_flags(&mut self, bytes: &HeaderBytes) -> HeaderResult<()> {
        self.flags = match HeaderFlags::from_bits(bytes[5]) {
            Some(flags) => flags,
            None => return Err(HeaderError::UnknownFlag),
        };

        Ok(())
    }
}

pub fn parse<R: Read>(reader: &mut BufReader<R>) -> HeaderResult<Header> {
    let mut bytes = [0u8; 10];
    try!(reader.read_exact(&mut bytes));

    if !is_valid(&bytes) {
        Err(HeaderError::NotID3)
    } else {
        Header::new_with_bytes(&bytes)
    }
}

fn is_valid(bytes: &HeaderBytes) -> bool {
    let identifier = &bytes[0..3];

    identifier == b"ID3"
}
