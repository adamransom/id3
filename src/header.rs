use std::io;
use std::io::{Read, BufReader};

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub revision: u8,
}

#[derive(Debug)]
pub struct Header {
    pub valid: bool,
    pub version: Version,
    pub unsynchronisation: bool,
    pub extended: bool,
    pub experimental: bool,
    pub has_footer: bool,
    pub size: u32,
}

impl Header {
    fn new() -> Header {
        Header {
            valid: false,
            version: Version { major: 0, revision: 0 },
            unsynchronisation: false,
            extended: false,
            experimental: false,
            has_footer: false,
            size: 0,
        }
    }
}

pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Header, io::Error> {
    let mut buf = [0u8; 10];
    try!(reader.read_exact(&mut buf));

    let mut header = Header::new();

    set_valid(&buf, &mut header);
    set_version(&buf, &mut header);
    set_unsynchronisation(&buf, &mut header);
    set_extended(&buf, &mut header);
    set_experimental(&buf, &mut header);
    set_has_footer(&buf, &mut header);
    set_size(&buf, &mut header);

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

fn set_has_footer(buf: &[u8; 10], header: &mut Header) {
    header.has_footer = buf[5] & 0b00010000 > 0;
}

fn set_size(buf: &[u8; 10], header: &mut Header) {
    header.size =
        (buf[6] as u32) << 27 |
        (buf[7] as u32) << 14 |
        (buf[8] as u32) << 7  |
        (buf[9] as u32);
}
