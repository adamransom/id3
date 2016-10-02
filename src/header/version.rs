/// A type representing the version of an ID3v2 tag.
#[derive(Debug, Default)]
pub struct Version {
    /// The major version. Currently only 3 is supported. _It must be less than 255._
    pub major: u8,
    /// The revision version. All revisions are backwards compatible. _It must be less than 255._
    pub revision: u8,
}
