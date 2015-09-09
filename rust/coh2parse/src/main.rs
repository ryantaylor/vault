// main.rs
#[macro_use]
extern crate log;
extern crate log4rs;

mod stream;
mod player;
mod replay;

use std::default::Default;
use std::path::Path;

use replay::Replay;

fn main() {
    // Initialize logging
    log4rs::init_file("log.toml", Default::default()).unwrap();

    // Create a path to the desired file
    let path = Path::new("/home/ryan/replays/test_base.rec");
    //let display = path.display();

    //let mut buff = [0xff, 0xff];
    //let mut buff: Vec<u8> = Vec::new();

    let mut replay = Replay::new(&path);
    replay.parse();

    /*let mut replay = ReplayFile::new(&path);
    replay.skip_u16();
    println!("{}", replay.read_u16());
    println!("{}", replay.read_text(8));*/
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





