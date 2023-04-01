//! `vault` library integration tests.

extern crate vault;

#[test]
fn parse() {
    let data = include_bytes!("/Users/ryantaylor/Downloads/release.rec");
    let replay = vault::parse_replay(data.to_vec());
    println!("{:#?}", replay);
}
