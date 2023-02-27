// //! `vault` library integration tests.
//
// extern crate vault;
//
// use std::path::Path;
//
// #[test]
// fn parse() {
//     let path_str = format!("{}/replays/bench.rec", env!("CARGO_MANIFEST_DIR"));
//     let path = Path::new(&path_str);
//     let replay = vault::parse_replay(&path, None).unwrap();
//     assert_eq!(replay.error, None);
// }
