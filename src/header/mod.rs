use std::io::{Read, BufReader};

mod error;

pub use self::error::HeaderError;

#[derive(Debug, Default)]
pub struct Version {
    pub major: u8,
    pub revision: u8,
}

#[derive(Debug, Default)]
pub struct Header {
    pub valid: bool,
    pub version: Version,
    pub unsynchronisation: bool,
    pub extended: bool,
    pub experimental: bool,
    pub size: u32,
}

pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Header, HeaderError> {
    let mut buf = [0u8; 10];
    try!(reader.read_exact(&mut buf));

    let mut header: Header = Default::default();

    set_valid(&buf, &mut header);
    set_version(&buf, &mut header);
    set_unsynchronisation(&buf, &mut header);
    set_extended(&buf, &mut header);
    set_experimental(&buf, &mut header);
    try!(set_size(&buf, &mut header));

    return Ok(header);
}

fn set_valid(buf: &[u8; 10], header: &mut Header) {
    let identifier = &buf[0..3];
    header.valid = identifier == b"ID3";
}

fn set_version(buf: &[u8; 10], header: &mut Header) {
    header.version.major = buf[3];
    header.version.revision = buf[4];
}

fn set_unsynchronisation(buf: &[u8; 10], header: &mut Header) {
    header.unsynchronisation = buf[5] & 0b10000000 > 0;
}

fn set_extended(buf: &[u8; 10], header: &mut Header) {
    header.extended = buf[5] & 0b01000000 > 0;
}

fn set_experimental(buf: &[u8; 10], header: &mut Header) {
    header.experimental = buf[5] & 0b00100000 > 0;
}

fn set_size(buf: &[u8; 10], header: &mut Header) -> Result<u32, HeaderError> {
    use utils;

    header.size = match utils::synchsafe_to_u32(&buf[6..10]) {
        Some(int) => int,
        None => 0,
    };

    return Err(HeaderError::InvalidSize(header.size));
}
