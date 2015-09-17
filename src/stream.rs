use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;
use std::u32;

use rustc_serialize::json::{self, ToJson, Json};

#[derive(Debug, RustcEncodable)]
pub enum StreamError {
    CursorWrap,
    CursorOutOfBounds,
    StringParseFailure,
    EmptyChar
}

#[derive(Debug, RustcEncodable)]
pub struct Stream {
    data: Vec<u8>,
    cursor: u32,
}

impl ToJson for Stream {
    fn to_json(&self) -> Json {
        Json::String(format!("{}", self.cursor))
    }
}

impl Stream {
    pub fn new(path: &Path) -> Stream {
        let meta = match fs::metadata(path) {
            Err(why) => panic!("couldn't read metadata for {}: {}", path.display(),
                                                                    Error::description(&why)),
            Ok(metadata) => metadata,
        };

        info!("{} contains {} bytes", path.display(), meta.len());

        if meta.len() >= u32::MAX as u64 {
            panic!("replay file size {} bytes surpasses max size {} bytes", meta.len(),
                                                                            u32::MAX - 1);
        }

        let mut replay = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(),
                                                       Error::description(&why)),
            Ok(file) => file,
        };

        let mut buff: Vec<u8> = Vec::with_capacity(meta.len() as usize);
        match replay.read_to_end(&mut buff) {
            Err(why) => panic!("couldn't read {}: {}", path.display(),
                                                       Error::description(&why)),
            Ok(_) => info!("{} opened and read into memory", path.display()),
        };

        info!("{} bytes read into memory", buff.len());

        Stream {
            data: buff,
            cursor: 0,
        }
    }

    pub fn skip_ahead(&mut self, pos: u32) -> Result<(), StreamError> {
        trace!("Stream::skip_ahead - skipping {} bytes", pos);
        if u32::MAX - self.cursor < pos {
            return Err(StreamError::CursorWrap);
        }

        if self.cursor + pos >= self.data.len() as u32 {
            warn!("cursor {} beyond max {}", self.cursor + pos, self.data.len() - 1);
        }

        debug!("Stream::skip_ahead - start cursor: {}", self.cursor);
        self.cursor += pos;
        debug!("Stream::skip_ahead - end cursor: {}", self.cursor);

        Ok(())
    }

    pub fn skip_back(&mut self, pos: u32) -> Result<(), StreamError> {
        trace!("Stream::skip_back - skipping {} bytes", pos);
        if pos > self.cursor {
            return Err(StreamError::CursorWrap);
        }

        if self.cursor - pos >= self.data.len() as u32 {
            warn!("cursor {} beyond max {}", self.cursor - pos, self.data.len() - 1);
        }

        debug!("Stream::skip_back - start cursor: {}", self.cursor);
        self.cursor -= pos;
        debug!("Stream::skip_back - end cursor: {}", self.cursor);

        Ok(())
    }

    pub fn seek(&mut self, pos: u32) {
        trace!("Stream::seek - seeking to {}", pos);
        if pos >= self.data.len() as u32 {
            warn!("cursor {} beyond max {}", pos, self.data.len() - 1);
        }

        debug!("Stream::skip_ahead - start cursor: {}", self.cursor);
        self.cursor = pos;
        debug!("Stream::skip_ahead - end cursor: {}", self.cursor);
    }

    pub fn read_u8(&mut self) -> Result<u8, StreamError> {
        trace!("Stream::read_u8 - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(StreamError::CursorOutOfBounds);
        }

        let result: u8 = self.data[self.cursor as usize];
        debug!("Stream::read_u8 - result: {}", result);
        self.cursor += 1;
        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16, StreamError> {
        trace!("Stream::read_u16 - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(StreamError::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < 2 {
            return Err(StreamError::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + 2) as usize];
        let result: u16 = ((stream[1] as u16) << 8) + (stream[0] as u16);
        debug!("Stream::read_u16 - result: {}", result);
        self.cursor += 2;
        Ok(result)
    }

    pub fn read_u32(&mut self) -> Result<u32, StreamError> {
        trace!("Stream::read_u32 - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(StreamError::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < 4 {
            return Err(StreamError::CursorOutOfBounds);
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

    pub fn read_u64(&mut self) -> Result<u64, StreamError> {
        trace!("Stream::read_u64 - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(StreamError::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < 8 {
            return Err(StreamError::CursorOutOfBounds);
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

    pub fn read_utf16(&mut self, len: u32) -> Result<String, StreamError> {
        trace!("Stream::read_utf16 - at cursor {} with len {}", self.cursor, len);
        if self.cursor >= self.data.len() as u32 {
            return Err(StreamError::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < len * 2 {
            return Err(StreamError::CursorOutOfBounds);
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

        let result = match String::from_utf16(&buff) {
            Err(_) => return Err(StreamError::StringParseFailure),
            Ok(val) => val
        };

        self.cursor += len * 2;

        debug!("Stream::read_utf16 - result: {}", result);
        Ok(result)
    }

    pub fn read_utf16_single(&mut self) -> Result<String, StreamError> {
        trace!("Stream::read_utf16_single - at cursor {}", self.cursor);
        if self.cursor >= self.data.len() as u32 {
            return Err(StreamError::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < 2 {
            return Err(StreamError::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + 2) as usize];
        let raw: u16 = ((stream[1] as u16) << 8) + (stream[0] as u16);

        if raw == 0 {
            error!("Stream::read_utf16_single - result: <empty>");
            self.cursor += 2;
            return Err(StreamError::EmptyChar);
        }

        let result = match String::from_utf16(&[raw]) {
            Err(_) => return Err(StreamError::StringParseFailure),
            Ok(val) => val
        };

        self.cursor += 2;

        debug!("Stream::read_utf16_single - result: {}", result);
        Ok(result)
    }

    pub fn read_utf8(&mut self, len: u32) -> Result<String, StreamError> {
        trace!("Stream::read_utf8 - at cursor {} with len {}", self.cursor, len);
        if self.cursor >= self.data.len() as u32 {
            return Err(StreamError::CursorOutOfBounds);
        }

        if self.data.len() as u32 - self.cursor < len {
            return Err(StreamError::CursorOutOfBounds);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + len) as usize];
        let mut stream_vec = Vec::with_capacity(len as usize);
        stream_vec.extend(stream.iter().cloned());

        let result = match String::from_utf8(stream_vec) {
            Err(_) => return Err(StreamError::StringParseFailure),
            Ok(val) => val,
        };

        self.cursor += len;

        debug!("Stream::read_utf8 - result: {}", result);
        Ok(result)
    }

    pub fn get_cursor_position(&self) -> u32 {
        return self.cursor;
    }

    pub fn cleanup(&mut self) {
        self.cursor = self.data.len() as u32;
        self.data = Vec::new();
    }
}