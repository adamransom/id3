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
    let mut bytes = [0u8; 10];
    try!(reader.read_exact(&mut bytes));

    let mut header: Header = Default::default();

    set_valid(&bytes, &mut header);
    set_version(&bytes, &mut header);
    set_unsynchronisation(&bytes, &mut header);
    set_extended(&bytes, &mut header);
    set_experimental(&bytes, &mut header);
    try!(set_size(&bytes, &mut header));

    return Ok(header);
}

fn set_valid(bytes: &[u8; 10], header: &mut Header) {
    let identifier = &bytes[0..3];
    header.valid = identifier == b"ID3";
}

fn set_version(bytes: &[u8; 10], header: &mut Header) {
    header.version.major = bytes[3];
    header.version.revision = bytes[4];
}

fn set_unsynchronisation(bytes: &[u8; 10], header: &mut Header) {
    header.unsynchronisation = bytes[5] & 0b10000000 > 0;
}

fn set_extended(bytes: &[u8; 10], header: &mut Header) {
    header.extended = bytes[5] & 0b01000000 > 0;
}

fn set_experimental(bytes: &[u8; 10], header: &mut Header) {
    header.experimental = bytes[5] & 0b00100000 > 0;
}

fn set_size(bytes: &[u8; 10], header: &mut Header) -> Result<u32, HeaderError> {
    use utils;

    header.size = match utils::synchsafe_to_u32(&bytes[6..10]) {
        Some(int) => int,
        None => 0,
    };

    return Err(HeaderError::InvalidSize(header.size));
}
