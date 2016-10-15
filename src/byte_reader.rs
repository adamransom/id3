use std::io;
use std::io::{Read, Seek, SeekFrom};

/// The `Skip` traits allows the reader to skip over a number of bytes of a stream.
pub trait Skip {
    /// Skips over `num_bytes` worth of bytes.
    ///
    /// `num_bytes` can be negative to 'skip' backwards.
    ///
    /// # Errors
    ///
    /// This will return an io::Error if the skip fails.
    fn skip(&mut self, num_bytes: i64) -> io::Result<u64>;
}

/// A struct for reading bytes from an underlying reader.
///
/// Contains a few methods to make it easier to work with reading bytes.
pub struct ByteReader<R> {
    peeked: Option<u8>,
    inner: R,
}

impl <R: Read> ByteReader<R> {
    /// Constructs a new ByteReader from an existing reader.
    pub fn new(reader: R) -> ByteReader<R> {
        ByteReader {
            inner: reader,
            peeked: None,
        }
    }

    /// Looks ahead at the next byte without 'consuming' it (i.e. the next read will still return
    /// the byte).
    ///
    /// # Examples
    ///
    /// ```
    /// # use id3::byte_reader::ByteReader;
    /// let buf = [1, 2, 3];
    /// let mut byte_reader = ByteReader::new(&buf[..]);
    ///
    /// assert_eq!(byte_reader.peek_byte().unwrap(), 1);
    /// ```
    /// # Errors
    ///
    /// This will return an io::Error if reading the next byte fails.
    pub fn peek_byte(&mut self) -> io::Result<u8> {
        match self.peeked {
            Some(byte) => Ok(byte),
            None => {
                let next_byte = try!(self.get_next_byte());
                self.peeked = Some(next_byte);

                Ok(next_byte)
            }
        }
    }

    /// Gets the next byte from the reader.
    ///
    /// # Errors
    ///
    /// This will return an io::Error if reading the next byte fails.
    fn get_next_byte(&mut self) -> io::Result<u8> {
        let mut bytes = [0u8; 1];
        try!(self.inner.read_exact(&mut bytes));

        Ok(bytes[0])
    }
}

impl <R: Seek> Skip for ByteReader<R> {
    fn skip(&mut self, num_bytes: i64) -> io::Result<u64> {
        self.inner.seek(SeekFrom::Current(num_bytes))
    }
}

impl <R: Read> Read for ByteReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Don't need to do anything if the buffer has no size
        if buf.len() == 0 {
            return Ok(0);
        }

        let offset = match self.peeked {
            Some(_) => 1,
            None => 0,
        };

        // If we have cached a previously peeked byte, put that byte in the buffer and clear the
        // cache
        if offset == 1 {
            buf[0] = self.peeked.unwrap();
            self.peeked = None;
        }

        if offset == 1 && buf.len() == 1 {
            // If buffer length is 1, then we can fill the buffer with only our previously peeked
            // byte
            Ok(1)
        } else {
            // Otherwise, read into the rest of the buffer and correct the returned size with the
            // offset
            Ok(try!(self.inner.read(&mut buf[offset..])) + offset)
        }
    }
}

#[cfg(test)]
mod byte_reader_tests {
    use std::io::Read;
    use super::ByteReader;

    #[test]
    fn it_peeks() {
        let buf = [1, 2, 3];
        let mut byte_reader = ByteReader::new(&buf[..]);

        assert_eq!(byte_reader.peek_byte().unwrap(), 1);
    }

    #[test]
    fn peek_is_idempotent() {
        let buf = [1, 2, 3];
        let mut byte_reader = ByteReader::new(&buf[..]);

        // First peek
        let _ = byte_reader.peek_byte();
        // Second peek
        assert_eq!(byte_reader.peek_byte().unwrap(), 1);
    }

    #[test]
    fn read_includes_peeked_byte() {
        let buf = [1, 2, 3];
        let mut byte_reader = ByteReader::new(&buf[..]);

        let mut fill_buf = [0; 3];

        let _ = byte_reader.peek_byte();
        let _ = byte_reader.read(&mut fill_buf);

        assert_eq!(fill_buf, buf);
    }
}
