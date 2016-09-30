use header::error::Error;
use header::version::Version;

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
pub struct Header {
    pub version: Version,
    pub flags: HeaderFlags,
    pub size: u32,
}

pub type HeaderBytes = [u8; 10];
pub type HeaderResult<T> = Result<T, Error>;

impl Header {
    pub fn new_with_bytes(bytes: &HeaderBytes) -> HeaderResult<Header> {
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
            Err(Error::InvalidSize)
        }
    }

    fn set_flags(&mut self, bytes: &HeaderBytes) -> HeaderResult<()> {
        self.flags = match HeaderFlags::from_bits(bytes[5]) {
            Some(flags) => flags,
            None => return Err(Error::UnknownFlag),
        };

        Ok(())
    }
}
