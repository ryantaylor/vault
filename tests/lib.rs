//! `vault` library integration tests.

extern crate vault;

use vault::Replay;

#[test]
fn parse_success() {
    let data = include_bytes!("/Users/ryantaylor/Downloads/release.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok())
}

#[test]
fn parse_failure() {
    let data = [1, 2, 3];
    let replay = Replay::from_bytes(&data);
    assert!(replay.is_err())
}
