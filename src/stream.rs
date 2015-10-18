//! A module that encloses the contents of the replay file in memory and performs various functions
//! on the resultant bytestream.

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;
use std::string::String;
use std::u32;

use Error;
use Result;

/// This type represents a Company of Heroes 2 replay file as a raw stream of bytes with an
/// associated file cursor.

#[derive(Debug, RustcEncodable)]
pub struct Stream {
    pub name: String,
    data: Vec<u8>,
    cursor: u32,
}

impl Stream {

    /// Constructs an empty Stream.

    pub fn new(name: &str) -> Stream {
        Stream {
            name: String::from(name),
            data: Vec::new(),
            cursor: 0,
        }
    }

    /// Constructs a new Stream using the file specified in path.

    pub fn from_file(path: &Path) -> Result<Stream> {
        let meta = try!(fs::metadata(path));

        info!("{} contains {} bytes", path.display(), meta.len());

        if meta.len() >= u32::MAX as u64 {
            return Err(Error::FileTooLarge);
        }

        let mut replay = try!(File::open(path));

        let mut buff: Vec<u8> = Vec::with_capacity(meta.len() as usize);
        try!(replay.read_to_end(&mut buff));

        info!("{} opened and read into memory", path.display());
        info!("{} bytes read into memory", buff.len());

        let name = path.to_string_lossy();
        let name = name.deref();

        Ok(Stream {
            name: String::from(name),
            data: buff,
            cursor: 0,
        })
    }

    /// Constructs a new Stream from the given byte vector.

    pub fn from_bytes(name: &str, bytes: Vec<u8>) -> Result<Stream> {
        if bytes.len() >= u32::MAX as usize {
            return Err(Error::FileTooLarge);
        }

        Ok(Stream {
            name: String::from(name),
            data: bytes,
            cursor: 0,
        })
    }

    /// Moves the file cursor pos positions forward.

    pub fn skip_ahead(&mut self, pos: u32) -> Result<()> {
        trace!("Stream::skip_ahead - skipping {} bytes", pos);
        if u32::MAX - self.cursor < pos {
            return Err(Error::CursorWrap);
        }

        if self.cursor + pos >= self.data.len() as u32 {
            warn!("cursor {} beyond max {}", self.cursor + pos, self.data.len() - 1);
        }

        debug!("Stream::skip_ahead - start cursor: {}", self.cursor);
        self.cursor += pos;
        debug!("Stream::skip_ahead - end cursor: {}", self.cursor);

        Ok(())
    }

    /// Moves the file cursor pos positions backward.

    pub fn skip_back(&mut self, pos: u32) -> Result<()> {
        trace!("Stream::skip_back - skipping {} bytes", pos);
        if pos > self.cursor {
            return Err(Error::CursorWrap);
        }

        if self.cursor - pos >= self.data.len() as u32 {
            warn!("cursor {} beyond max {}", self.cursor - pos, self.data.len() - 1);
        }

        debug!("Stream::skip_back - start cursor: {}", self.cursor);
        self.cursor -= pos;
        debug!("Stream::skip_back - end cursor: {}", self.cursor);

        Ok(())
    }

    /// Moves the file cursor to the position given in pos.

    pub fn seek(&mut self, pos: u32) {
        trace!("Stream::seek - seeking to {}", pos);
        if pos >= self.data.len() as u32 {
            warn!("cursor {} beyond max {}", pos, self.data.len() - 1);
        }

        debug!("Stream::skip_ahead - start cursor: {}", self.cursor);
        self.cursor = pos;
        debug!("Stream::skip_ahead - end cursor: {}", self.cursor);
    }

    /// Reads an 8-bit (1-byte) unsigned integer at the current cursor position, then moves the
    /// cursor ahead 1 position.

    pub fn read_u8(&mut self) -> Result<u8> {
        trace!("Stream::read_u8 - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(Error::CursorOutOfBounds);
        }

        let result: u8 = self.data[self.cursor as usize];
        debug!("Stream::read_u8 - result: {}", result);
        self.cursor += 1;
        Ok(result)
    }

    /// Reads a 16-bit (2-byte) unsigned integer at the current cursor position, then moves the
    /// cursor ahead 2 positions.
    ///
    /// This method reads a little endian integer. When called on the byte stream 01 00 the return
    /// value will be 1.

    pub fn read_u16(&mut self) -> Result<u16> {
        trace!("Stream::read_u16 - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(Error::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < 2 {
            return Err(Error::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + 2) as usize];
        let result: u16 = ((stream[1] as u16) << 8) + (stream[0] as u16);
        debug!("Stream::read_u16 - result: {}", result);
        self.cursor += 2;
        Ok(result)
    }

    /// Reads a 32-bit (4-byte) unsigned integer at the current cursor position, then moves the
    /// cursor ahead 4 positions.
    ///
    /// This method reads a little endian integer. When called on the byte stream 01 00 00 00 the
    /// return value will be 1.

    pub fn read_u32(&mut self) -> Result<u32> {
        trace!("Stream::read_u32 - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(Error::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < 4 {
            return Err(Error::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + 4) as usize];
        let result: u32 = ((stream[3] as u32) << 24) + 
                          ((stream[2] as u32) << 16) +
                          ((stream[1] as u32) << 8) +
                          (stream[0] as u32);
        debug!("Stream::read_u32 - result: {}", result);
        self.cursor += 4;
        Ok(result)
    }

    /// Reads a 64-bit (8-byte) unsigned integer at the current cursor position, then moves the
    /// cursor ahead 8 positions.
    ///
    /// This method reads a little endian integer. When called on the byte stream
    /// 01 00 00 00 00 00 00 00 the return value will be 1.

    pub fn read_u64(&mut self) -> Result<u64> {
        trace!("Stream::read_u64 - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(Error::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < 8 {
            return Err(Error::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + 8) as usize];
        let result: u64 = ((stream[7] as u64) << 56) +
                          ((stream[6] as u64) << 48) +
                          ((stream[5] as u64) << 40) +
                          ((stream[4] as u64) << 32) +
                          ((stream[3] as u64) << 24) + 
                          ((stream[2] as u64) << 16) +
                          ((stream[1] as u64) << 8) +
                          (stream[0] as u64);
        debug!("Stream::read_u64 - result: {}", result);
        self.cursor += 8;
        Ok(result)
    }

    /// Reads a sequence of 16-bit unsigned integers that represent 16-bit Unicode characters, then
    /// moves the cursor ahead len * 2 positions.

    pub fn read_utf16(&mut self, len: u32) -> Result<String> {
        trace!("Stream::read_utf16 - at cursor {} with len {}", self.cursor, len);
        if self.cursor >= self.data.len() as u32 {
            return Err(Error::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < len * 2 {
            return Err(Error::CursorOutOfBounds);
        }

        let mut buff: Vec<u16> = Vec::with_capacity(len as usize);
        let stream = &self.data[self.cursor as usize..(self.cursor + len * 2) as usize];
        let mut first = true;
        let mut idx = 0;

        for val in stream.iter() {
            if first {
                buff.push(*val as u16);
                first = false;
            }
            else {
                buff[idx] += (*val as u16) << 8;
                first = true;
                idx += 1;
            }
        }

        let result = try!(String::from_utf16(&buff));

        self.cursor += len * 2;

        debug!("Stream::read_utf16 - result: {}", result);
        Ok(result)
    }

    /// Reads a single 16-bit Unicode character and returns an error if the character read is
    /// empty. The cursor is then moved ahead 2 positions.

    pub fn read_utf16_single(&mut self) -> Result<String> {
        trace!("Stream::read_utf16_single - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(Error::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < 2 {
            return Err(Error::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + 2) as usize];
        let raw: u16 = ((stream[1] as u16) << 8) + (stream[0] as u16);

        if raw == 0 {
            error!("Stream::read_utf16_single - result: <empty>");
            self.cursor += 2;
            return Err(Error::EmptyChar);
        }

        let result = try!(String::from_utf16(&[raw]));

        self.cursor += 2;

        debug!("Stream::read_utf16_single - result: {}", result);
        Ok(result)
    }

    /// Reads a sequence of 8-bit unsigned integers that represent 8-bit Unicode characters, then
    /// moves the cursor ahead len positions.

    pub fn read_utf8(&mut self, len: u32) -> Result<String> {
        trace!("Stream::read_utf8 - at cursor {} with len {}", self.cursor, len);
        if self.cursor >= self.data.len() as u32 {
            return Err(Error::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < len {
            return Err(Error::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + len) as usize];
        let mut stream_vec = Vec::with_capacity(len as usize);
        stream_vec.extend(stream.iter().cloned());

        let result = try!(String::from_utf8(stream_vec));

        self.cursor += len;

        debug!("Stream::read_utf8 - result: {}", result);
        Ok(result)
    }

    /// Reads the given number of bytes at the current cursor location into a vector and returns
    /// it without moving the cursor. Mainly useful for debugging things like command byte
    /// sequences.

    pub fn read_to_vec(&self, len: u32) -> Result<Vec<u8>> {
        trace!("Stream::read_to_vec - at cursor {} with len {}", self.cursor, len);

        if self.cursor >= self.data.len() as u32 {
            return Err(Error::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < len {
            return Err(Error::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + len) as usize];
        let mut stream_vec = Vec::with_capacity(len as usize);
        stream_vec.extend(stream.iter().cloned());

        Ok(stream_vec)
    }

    /// Returns the current position of the cursor.

    pub fn get_cursor_position(&self) -> u32 {
        self.cursor
    }

    /// Returns the filename of the loaded replay.

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Clears the vector of data bytes loaded from file and sets the cursor to the length of the
    /// file.
    ///
    /// This is generally done at the end of parsing because we no longer have any use for the raw
    /// byte stream, and keeping it makes serializing the Replay type messy.

    pub fn cleanup(&mut self) {
        self.cursor = self.data.len() as u32;
        self.data = Vec::new();
    }
}