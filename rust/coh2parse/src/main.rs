// main.rs
#[macro_use]
extern crate log;
extern crate log4rs;

mod stream;
mod player;
mod replay;
mod equippable;

use std::default::Default;
use std::path::Path;

use replay::Replay;

fn main() {
    // Initialize logging
    log4rs::init_file("log.toml", Default::default()).unwrap();

    // Create a path to the desired file
    let path = Path::new("/home/ryan/replays/angoville_1v1.rec");

    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn angoville_1v1() {
    let path = Path::new("/home/ryan/replays/angoville_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn arnhem_1v1() {
    let path = Path::new("/home/ryan/replays/arnhem_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn city_17_4v4() {
    let path = Path::new("/home/ryan/replays/city_17_4v4.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn crossing_2v2() {
    let path = Path::new("/home/ryan/replays/crossing_2v2.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn dusseldorf_2v2() {
    let path = Path::new("/home/ryan/replays/dusseldorf_2v2.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn einhoven_1v1() {
    let path = Path::new("/home/ryan/replays/einhoven_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn essen_4v4() {
    let path = Path::new("/home/ryan/replays/essen_4v4.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn ettelbruck_2v2() {
    let path = Path::new("/home/ryan/replays/ettelbruck_2v2.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn faymonville_1v1() {
    let path = Path::new("/home/ryan/replays/faymonville_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn kharkov_1v1() {
    let path = Path::new("/home/ryan/replays/kharkov_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn kholodny_1v1() {
    let path = Path::new("/home/ryan/replays/kholodny_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn kholodny_winter_1v1() {
    let path = Path::new("/home/ryan/replays/kholodny_winter_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn la_gleize_1v1() {
    let path = Path::new("/home/ryan/replays/la_gleize_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn langres_1v1() {
    let path = Path::new("/home/ryan/replays/langres_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn lazenrath_3v3() {
    let path = Path::new("/home/ryan/replays/lazenrath_3v3.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn lienne_2v2() {
    let path = Path::new("/home/ryan/replays/lienne_2v2.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn lierneux_2v2() {
    let path = Path::new("/home/ryan/replays/lierneux_2v2.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn lorch_4v4() {
    let path = Path::new("/home/ryan/replays/lorch_4v4.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn minsk_1v1() {
    let path = Path::new("/home/ryan/replays/minsk_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn moscow_2v2() {
    let path = Path::new("/home/ryan/replays/moscow_2v2.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn pavlov_3v4() {
    let path = Path::new("/home/ryan/replays/pavlov_3v4.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn rails_2v2() {
    let path = Path::new("/home/ryan/replays/rails_2v2.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn rzhev_winter_2v2() {
    let path = Path::new("/home/ryan/replays/rzhev_winter_2v2.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn semois_1v1() {
    let path = Path::new("/home/ryan/replays/semois_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn semois_winter_1v1() {
    let path = Path::new("/home/ryan/replays/semois_winter_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}

#[test]
fn sittard_1v1() {
    let path = Path::new("/home/ryan/replays/sittard_1v1.rec");
    let mut replay = Replay::new(&path);
    replay.parse();
}