//! `vault` library unit tests.

#[cfg(feature = "dev")]
use test::Bencher;

use command::*;
#[cfg(feature = "dev")]
use config::*;
use error::*;
use item::*;
use stream::*;
use std::{u32, u64};
#[cfg(feature = "dev")]
use std::path::Path;

// mod command

#[test]
fn cmdtype_bounds() {
    assert_eq!(CmdType::from_u8(0).unwrap(), CmdType::CMD_DefaultAction);   // lower
    assert_eq!(CmdType::from_u8(106).unwrap(), CmdType::DCMD_COUNT);        // upper
    assert_eq!(CmdType::from_u8(107), None);                                // beyond
}

// mod item

// #[test]
// fn item_update_id() {
//     let mut item = Item::new(ItemType::Commander);
//     item.update_id(0, 0);
//     assert_eq!(item.id, 0);                     // lower
//     item.update_id(u32::MAX, u32::MAX);
//     assert_eq!(item.id, u64::MAX);              // upper
//     item.update_id(0x23578654, 0x82123245);
//     assert_eq!(item.id, 0x2357865482123245);    // middle
//     item.update_id(0x82123245, 0x23578654);
//     assert_eq!(item.id, 0x8212324523578654);    // middle reverse
// }

// mod stream

#[test]
#[ignore]
fn stream_from_bytes_upper() {
    // this test needs 4GiB of available memory to run
    let bytes: Vec<u8> = vec![0; (u32::MAX - 1) as usize];
    Stream::from_bytes("test", bytes).unwrap();
}

#[test]
#[should_panic(expected = "FileTooLarge")]
#[ignore]
fn stream_from_bytes_beyond_upper() {
    // this test needs 4GiB of available memory to run
    let bytes: Vec<u8> = vec![0; u32::MAX as usize];
    Stream::from_bytes("test", bytes).unwrap();
}

#[test]
fn stream_from_bytes_lower() {
    let bytes: Vec<u8> = vec![];
    Stream::from_bytes("test", bytes).unwrap();
}

#[test]
fn seek() {
    let bytes: Vec<u8> = vec![0; 100];
    let mut stream = Stream::from_bytes("test", bytes).unwrap();

    stream.seek(0);
    assert_eq!(stream.get_cursor_position(), 0);
    stream.seek(1);
    assert_eq!(stream.get_cursor_position(), 1);
    stream.seek(99);
    assert_eq!(stream.get_cursor_position(), 99);
    stream.seek(100);
    assert_eq!(stream.get_cursor_position(), 100);
    stream.seek(u32::MAX);
    assert_eq!(stream.get_cursor_position(), u32::MAX);
}

#[test]
fn stream_skip_ahead() {
    let bytes: Vec<u8> = vec![0; 100];
    let mut stream = Stream::from_bytes("test", bytes).unwrap();

    stream.skip_ahead(0).unwrap();
    assert_eq!(stream.get_cursor_position(), 0);
    stream.skip_ahead(1).unwrap();
    assert_eq!(stream.get_cursor_position(), 1);
    stream.skip_ahead(5).unwrap();
    assert_eq!(stream.get_cursor_position(), 6);
    stream.skip_ahead(93).unwrap();
    assert_eq!(stream.get_cursor_position(), 99);
    stream.skip_ahead(0).unwrap();
    assert_eq!(stream.get_cursor_position(), 99);

    stream.seek(0);
    assert_eq!(stream.get_cursor_position(), 0);

    stream.skip_ahead(u32::MAX).unwrap();
    assert_eq!(stream.get_cursor_position(), u32::MAX);
    match stream.skip_ahead(1) {
        Err(Error::CursorWrap) => {},
        _ => panic!()
    }
}

#[test]
fn stream_skip_back() {
    let bytes: Vec<u8> = vec![0; 100];
    let mut stream = Stream::from_bytes("test", bytes).unwrap();

    stream.seek(99);
    assert_eq!(stream.get_cursor_position(), 99);
    stream.skip_back(1).unwrap();
    assert_eq!(stream.get_cursor_position(), 98);
    stream.skip_back(5).unwrap();
    assert_eq!(stream.get_cursor_position(), 93);
    stream.skip_back(93).unwrap();
    assert_eq!(stream.get_cursor_position(), 0);
    stream.skip_back(0).unwrap();
    assert_eq!(stream.get_cursor_position(), 0);

    match stream.skip_back(1) {
        Err(Error::CursorWrap) => {},
        _ => panic!()
    }
}

#[test]
fn stream_read_u8() {
    let bytes: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let mut stream = Stream::from_bytes("test", bytes).unwrap();

    assert_eq!(stream.read_u8().unwrap(), 0x00);
    assert_eq!(stream.read_u8().unwrap(), 0x01);
    assert_eq!(stream.read_u8().unwrap(), 0x02);
    assert_eq!(stream.read_u8().unwrap(), 0x03);
    assert_eq!(stream.read_u8().unwrap(), 0x04);
    assert_eq!(stream.read_u8().unwrap(), 0x05);

    match stream.read_u8() {
        Err(Error::CursorOutOfBounds) => {},
        _ => panic!()
    }
}

#[test]
fn stream_read_u16() {
    let bytes: Vec<u8> = vec![0x00, 0x01,
                              0x02, 0x03,
                              0x04, 0x05];
    let mut stream = Stream::from_bytes("test", bytes).unwrap();

    assert_eq!(stream.read_u16().unwrap(), 0x0100);
    assert_eq!(stream.read_u16().unwrap(), 0x0302);
    assert_eq!(stream.read_u16().unwrap(), 0x0504);

    match stream.read_u16() {
        Err(Error::CursorOutOfBounds) => {},
        _ => panic!()
    }

    stream.skip_back(1).unwrap();

    match stream.read_u16() {
        Err(Error::CursorOutOfBounds) => {},
        _ => panic!()
    }

    stream.skip_back(1).unwrap();

    assert_eq!(stream.read_u16().unwrap(), 0x0504);
}

#[test]
fn stream_read_u32() {
    let bytes: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03,
                              0x04, 0x05, 0x06, 0x07,
                              0x08, 0x09, 0x0A, 0x0B];
    let mut stream = Stream::from_bytes("test", bytes).unwrap();

    assert_eq!(stream.read_u32().unwrap(), 0x03020100);
    assert_eq!(stream.read_u32().unwrap(), 0x07060504);
    assert_eq!(stream.read_u32().unwrap(), 0x0B0A0908);

    match stream.read_u32() {
        Err(Error::CursorOutOfBounds) => {},
        _ => panic!()
    }

    stream.skip_back(3).unwrap();

    match stream.read_u32() {
        Err(Error::CursorOutOfBounds) => {},
        _ => panic!()
    }

    stream.skip_back(1).unwrap();

    assert_eq!(stream.read_u32().unwrap(), 0x0B0A0908);
}

#[test]
fn stream_read_u64() {
    let bytes: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                              0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    let mut stream = Stream::from_bytes("test", bytes).unwrap();

    assert_eq!(stream.read_u64().unwrap(), 0x0706050403020100);
    assert_eq!(stream.read_u64().unwrap(), 0x0F0E0D0C0B0A0908);

    match stream.read_u64() {
        Err(Error::CursorOutOfBounds) => {},
        _ => panic!()
    }

    stream.skip_back(7).unwrap();

    match stream.read_u64() {
        Err(Error::CursorOutOfBounds) => {},
        _ => panic!()
    }

    stream.skip_back(1).unwrap();

    assert_eq!(stream.read_u64().unwrap(), 0x0F0E0D0C0B0A0908);
}

#[test]
fn stream_cleanup() {
    let bytes: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let mut stream = Stream::from_bytes("test", bytes).unwrap();

    stream.cleanup();

    assert_eq!(stream.get_cursor_position(), 6);
}

#[cfg(feature = "dev")]
#[bench]
fn bench_config_bare(b: &mut Bencher) {
    b.iter(|| {
        let path_str = format!("{}/replays/bench.rec", env!("CARGO_MANIFEST_DIR"));
        let config = Config::new(false, false, false);
        let path = Path::new(&path_str);
        ::parse_replay(&path, config).unwrap();
    });
}

#[cfg(feature = "dev")]
#[bench]
fn bench_config_default(b: &mut Bencher) {
    b.iter(|| {
        let path_str = format!("{}/replays/bench.rec", env!("CARGO_MANIFEST_DIR"));
        let path = Path::new(&path_str);
        ::parse_replay(&path, None).unwrap();
    });
}

#[cfg(feature = "dev")]
#[bench]
fn bench_config_all(b: &mut Bencher) {
    b.iter(|| {
        let path_str = format!("{}/replays/bench.rec", env!("CARGO_MANIFEST_DIR"));
        let config = Config::new(true, true, true);
        let path = Path::new(&path_str);
        ::parse_replay(&path, config).unwrap();
    });
}