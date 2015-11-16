//! `vault` library integration tests.

extern crate vault;

use std::path::Path;

use vault::Vault;

#[test]
fn parse() {
    let path_str = format!("{}/replays/bench.rec", env!("CARGO_MANIFEST_DIR"));
    let path = Path::new(&path_str);
    let vault = Vault::parse(&path).unwrap();
    for replay in &vault.replays {
        assert_eq!(replay.error, None);
    }
}