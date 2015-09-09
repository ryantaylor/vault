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
    let path = Path::new("/home/ryan/replays/test_base_4v4_cpu.rec");

    let mut replay = Replay::new(&path);
    replay.parse();
}





