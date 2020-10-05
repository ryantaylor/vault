use std::fs::File;
use std::io::Read;
use std::path::Path;
use test::Bencher;

use nom::{le_u8, le_u16, le_u32, IResult};
use nom::types::CompleteByteSlice;

use new_replay::NewReplay;
use parser::{orig_le_u16, g_le_u8, g_le_u16, g_le_u32, g_le_u64, cbs_le_u16, match_utf8, match_version, match_terminated_utf16};

const GAME_TYPE: &'static str = "COH2_REC";
const CHUNKY_NAME: &'static str = "Relic Chunky";

const CHUNKY_TYPE: u32 = 0x1A0A0D;
const CHUNKY_VERSION: u32 = 0x3;

pub fn parse(path: &Path) -> bool {
    let mut file = File::open(path).unwrap();
    let mut buff: Vec<u8> = Vec::new();
    file.read_to_end(&mut buff).unwrap();

    let (remaining, replay) = parse_header(&buff).unwrap();
    true
}

named!(parse_header<(u16, &str, String)>,
    do_parse!(
        version: match_version >>
        game_type: apply!(match_utf8, GAME_TYPE) >>
        timestamp: match_terminated_utf16 >>
        many_m_n!(7, 7, verify!(le_u32, |n: u32| n == 0)) >>
        (version, game_type, timestamp)
    )
);

named!(parse_chunky<bool>,
    do_parse!(
        apply!(match_utf8, CHUNKY_NAME) >>
        verify!(le_u32, |n| n == CHUNKY_TYPE) >>
        verify!(le_u32, |n| n == CHUNKY_VERSION) >>
        verify!(le_u32, |n| n == 0x1) >>
        verify!(le_u32, |n| n == 0x24) >>
        verify!(le_u32, |n| n == 0x1C) >>
        verify!(le_u32, |n| n == 0x1) >>
        (true)
    )
);

named!(test_eof<CompleteByteSlice, bool>,
    do_parse!(
        // many_till!(g_le_u16, eof!()) >>
        count!(g_le_u8, 2000000) >>
        (true)
    )
);

named!(test_eof_slice<bool>,
    do_parse!(
        count!(g_le_u8, 2000000) >>
        (true)
    )
);

// fn test_eof_long(input: CompleteByteSlice) -> IResult<CompleteByteSlice, CompleteByteSlice> {
//     many_till!(input, take!(1), eof!())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_ok() {
        let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
        let (_, (version, game_type, timestamp)) = parse_header(&buff).unwrap();
        assert_eq!(version, 20297);
        assert_eq!(game_type, "COH2_REC");
        assert_eq!(timestamp, "11/7/2015 1:16 AM");
    }

    #[test]
    fn test_parse_chunky_ok() {
        let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
        let (remaining, _) = parse_header(&buff).unwrap();
        let (_, result) = parse_chunky(remaining).unwrap();
        assert!(result);
    }

    #[test]
    fn test_parse_eof() {
        let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
        let (remaining, _) = test_eof(CompleteByteSlice(&buff)).unwrap();
    }

    #[bench]
    fn bench_parse_eof(b: &mut Bencher) {
        let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
        b.iter(|| {
            test_eof(CompleteByteSlice(&buff)).unwrap()
            // println!("{:?}", remaining);
            // println!("{:?}", val);
        });
    }

    #[bench]
    fn bench_parse_eof_slice(b: &mut Bencher) {
        let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
        b.iter(|| {
            test_eof_slice(&buff).unwrap()
            // println!("{:?}", remaining);
        });
    }

    fn read_into_buffer(path: &Path) -> Vec<u8> {
        let mut file = File::open(path).unwrap();
        let mut buff: Vec<u8> = Vec::new();
        file.read_to_end(&mut buff).unwrap();
        buff
    }
}
