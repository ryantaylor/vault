// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
//use std::io::SeekFrom;
use std::path::Path;
use std::string::String;

fn main() {
    // Create a path to the desired file
    let path = Path::new("/home/ryan/replays/test.rec");
    //let display = path.display();

    //let mut buff = [0xff, 0xff];
    //let mut buff: Vec<u8> = Vec::new();

    let mut replay = ReplayFile::new(&path);
    replay.skip_u16();
    println!("{}", replay.read_u16());
    println!("{}", replay.read_text(8));
    /*let val: u8 = 1;

    println!("{}", replay.test(val));*/

    // Open the path in read-only mode, returns `io::Result<File>`
    /*let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    match file.read_to_end(&mut buff) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}\n{}\n", display, buff[2], buff[3]),
    }

    let slice = &buff[2..3+1];

    println!("{}\n{}", testfn(slice[0]), slice[1]);
    println!("{}", ((slice[0] as u16) << 8));*/

    // Read the file contents into a string, returns `io::Result<usize>`
    //let mut s = String::new();
    /*match file.read(&mut buff) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}\n{}\n", display, buff[0], buff[1]),
    }*/

    /*match file.seek(SeekFrom::Start(0)) {
        Err(why) => panic!("failed to seek: {}", Error::description(&why)),
        Ok(_) => println!("seek successful"),
    }*/

    /*match file.read(&mut buff) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}\n{}\n", display, buff[0], buff[1]),
    }*/

    /*let mut result_u32;
    let mut result_u16: u16 = buff[1];
    result_u16 = result_u16 << 8;

    result_u32 = buff[0] + result_u16;
    println!("{}", result_u32);*/
    // `file` goes out of scope, and the "hello.txt" file gets closed
}

struct ReplayFile {
    file: File,
    data: Vec<u8>,
    cursor: i32,
}

impl ReplayFile {
    fn new(path: &Path) -> ReplayFile {
        let mut replay = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(),
                                                       Error::description(&why)),
            Ok(file) => file,
        };

        let mut buff: Vec<u8> = Vec::new();
        match replay.read_to_end(&mut buff) {
            Err(why) => panic!("couldn't read {}: {}", path.display(),
                                                       Error::description(&why)),
            Ok(_) => println!("{} opened and read into memory", path.display()),
        };

        ReplayFile {
            file: replay,
            data: buff,
            cursor: 0,
        }
    }

    fn skip(&mut self, pos: i32) {
        self.cursor += pos;

        if self.cursor < 0 {
            panic!("ReplayFile::skip - cursor {} is less than minimum value 0", self.cursor);
        }
    }

    fn skip_u8(&mut self) {
        self.skip(1);
    }

    fn skip_u16(&mut self) {
        self.skip(2);
    }

    fn skip_u32(&mut self) {
        self.skip(4);
    }

    fn skip_u64(&mut self) {
        self.skip(8);
    }

    fn seek(&mut self, pos: i32) {
        if pos < 0 {
            panic!("ReplayFile::seek - cursor {} is less than minimum value 0", pos);
        }

        self.cursor = pos;
    }

    fn read_u8(&mut self) -> u8 {
        let result: u8 = self.data[self.cursor as usize];
        self.cursor += 1;
        result
    }

    fn read_u16(&mut self) -> u16 {
        let stream = &self.data[self.cursor as usize..(self.cursor + 2) as usize];
        let result: u16 = ((stream[1] as u16) << 8) + (stream[0] as u16);
        self.cursor += 2;
        result
    }

    fn read_u32(&mut self) -> u32 {
        let stream = &self.data[self.cursor as usize..(self.cursor + 4) as usize];
        let result: u32 = ((stream[3] as u32) << 24) + 
                          ((stream[2] as u32) << 16) +
                          ((stream[1] as u32) << 8) +
                          (stream[0] as u32);
        self.cursor += 4;
        result
    }

    fn read_text(&mut self, len: i32) -> String {
        if len < 0 {
            panic!("length {} is less than minimum length 0", len);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + len) as usize];
        let mut stream_vec = Vec::new();
        stream_vec.extend(stream.iter().cloned());

        let result = match String::from_utf8(stream_vec) {
            Err(why) => panic!("couldn't read text, cursor {} len {}: {}", self.cursor,
                                                                           len,
                                                                           Error::description(&why)),
            Ok(text) => text,
        };

        self.cursor += len;
        result
    }
}