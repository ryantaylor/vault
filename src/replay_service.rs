use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::option::Option;
// use test::Bencher;

// use nom::{le_u8, le_u16, le_u32, IResult};
// use nom::types::CompleteByteSlice;

use nom::IResult;
use nom::branch::{alt};
use nom::bytes::complete::{tag};
use nom::combinator::{map, peek};
use nom::multi::{many0};
use nom::number::complete::{le_u32};
use nom::sequence::{preceded, tuple};

use new_replay::NewReplay;
// use parser::{orig_le_u16, g_le_u8, g_le_u16, g_le_u32, g_le_u64, cbs_le_u16, match_utf8, match_version, match_terminated_utf16};

use parser::{parse_utf8, parse_version, parse_game_type, parse_utf16, take_zeroes, verify_le_u32};

const GAME_TYPE: &'static str = "COH2_REC";
const CHUNKY_NAME: &'static str = "Relic Chunky";

const CHUNKY_TYPE: u32 = 0x1A0A0D;
const CHUNKY_VERSION: u32 = 0x3;

pub fn parse(path: &Path) -> bool {
    let mut file = File::open(path).unwrap();
    let mut buff: Vec<u8> = Vec::new();
    file.read_to_end(&mut buff).unwrap();

    let (remaining, replay) = parse_replay(&buff).unwrap();
    true
}

fn parse_replay(input: &[u8]) -> IResult<&[u8], NewReplay> {
    let (input, (version, game_type, timestamp)) = tuple((parse_version, parse_game_type, parse_utf16))(input)?;
    Ok((input, NewReplay::new(version, game_type, timestamp)))
}

struct Header {
    pub version: u32,
    pub game_type: String,
    pub timestamp: String
}

fn parse_header(input: &[u8]) -> IResult<&[u8], Header> {
    map(
        tuple((
            le_u32,
            parse_utf8(8),
            parse_utf16,
            take_zeroes
        )),
        |(
            version,
            game_type,
            timestamp,
            _
        )| {
            Header {
                version,
                timestamp,
                game_type: game_type.to_owned()
            }
        }
    )(input)
}

struct RelicChunky {
    pub name: String,
    pub signature: u32,
    pub major_version: u32,
    pub minor_version: u32, // maybe?
    pub chunk_offset: u32, // bytes from start of chunky to start of first member chunk
    pub unknown_offset: u32, // usually 0x1C
    pub unknown_id: u32, // usually 0x1
    pub chunks: Vec<RelicChunk>
}

fn parse_chunky(input: &[u8]) -> IResult<&[u8], RelicChunky> {
    map(
        tuple((
            tag("Relic Chunky"),
            verify_le_u32(0x1A0A0D),
            verify_le_u32(0x3),
            verify_le_u32(0x3),
            le_u32,
            le_u32,
            le_u32,
            many0(parse_chunk)
        )),
        |(
            name,
            signature,
            major_version,
            minor_version,
            chunk_offset,
            unknown_offset,
            unknown_id,
            chunks
        )| {
            RelicChunky {
                name: String::from_utf8_lossy(name).into_owned(),
                signature,
                major_version,
                minor_version,
                chunk_offset,
                unknown_offset,
                unknown_id,
                chunks
            }
        }
    )(input)
}

struct RelicChunk {
    pub chunk_kind: String,
    pub chunk_type: String
}

fn parse_chunk(input: &[u8]) -> IResult<&[u8], RelicChunk> {
    alt((
        parse_data_chunk, parse_folder_chunk
    ))(input)
}

fn parse_folder_chunk(input: &[u8]) -> IResult<&[u8], RelicChunk> {
    preceded(
        peek(tag("FOLD")),
        alt((
            parse_foldinfo_chunk,
            parse_foldpost_chunk
        ))
    )(input)
}

fn parse_foldinfo_chunk(input: &[u8]) -> IResult<&[u8], RelicChunk> {
    map(
        tuple((
            tag("FOLD"),
            tag("INFO")
        )),
        |(
            chunk_kind,
            chunk_type
        )| {
            RelicChunk {
                chunk_kind: String::from_utf8_lossy(chunk_kind).into_owned(),
                chunk_type: String::from_utf8_lossy(chunk_type).into_owned()
            }
        }
    )(input)
}

fn parse_foldpost_chunk(input: &[u8]) -> IResult<&[u8], RelicChunk> {
    map(
        tuple((
            tag("FOLD"),
            tag("POST")
        )),
        |(
            chunk_kind,
            chunk_type
        )| {
            RelicChunk {
                chunk_kind: String::from_utf8_lossy(chunk_kind).into_owned(),
                chunk_type: String::from_utf8_lossy(chunk_type).into_owned()
            }
        }
    )(input)
}

fn parse_data_chunk(input: &[u8]) -> IResult<&[u8], RelicChunk> {
    preceded(
        peek(tag("DATA")),
        alt((
            parse_datasdsc_chunk,
            parse_datadata_chunk,
            parse_dataplas_chunk
        ))
    )(input)
}

fn parse_datasdsc_chunk(input: &[u8]) -> IResult<&[u8], RelicChunk> {
    map(
        tuple((
            tag("DATA"),
            tag("SDSC")
        )),
        |(
            chunk_kind,
            chunk_type
        )| {
            RelicChunk {
                chunk_kind: String::from_utf8_lossy(chunk_kind).into_owned(),
                chunk_type: String::from_utf8_lossy(chunk_type).into_owned()
            }
        }
    )(input)
}

fn parse_datadata_chunk(input: &[u8]) -> IResult<&[u8], RelicChunk> {
    map(
        tuple((
            tag("DATA"),
            tag("DATA")
        )),
        |(
            chunk_kind,
            chunk_type
        )| {
            RelicChunk {
                chunk_kind: String::from_utf8_lossy(chunk_kind).into_owned(),
                chunk_type: String::from_utf8_lossy(chunk_type).into_owned()
            }
        }
    )(input)
}

fn parse_dataplas_chunk(input: &[u8]) -> IResult<&[u8], RelicChunk> {
    map(
        tuple((
            tag("DATA"),
            tag("PLAS")
        )),
        |(
            chunk_kind,
            chunk_type
        )| {
            RelicChunk {
                chunk_kind: String::from_utf8_lossy(chunk_kind).into_owned(),
                chunk_type: String::from_utf8_lossy(chunk_type).into_owned()
            }
        }
    )(input)
}

// named!(parse_header<(u16, &str, String)>,
//     do_parse!(
//         version: match_version >>
//         game_type: apply!(match_utf8, GAME_TYPE) >>
//         timestamp: match_terminated_utf16 >>
//         many_m_n!(7, 7, verify!(le_u32, |n: u32| n == 0)) >>
//         (version, game_type, timestamp)
//     )
// );

// named!(parse_chunky<bool>,
//     do_parse!(
//         apply!(match_utf8, CHUNKY_NAME) >>
//         verify!(le_u32, |n| n == CHUNKY_TYPE) >>
//         verify!(le_u32, |n| n == CHUNKY_VERSION) >>
//         verify!(le_u32, |n| n == 0x1) >>
//         verify!(le_u32, |n| n == 0x24) >>
//         verify!(le_u32, |n| n == 0x1C) >>
//         verify!(le_u32, |n| n == 0x1) >>
//         (true)
//     )
// );

// named!(test_eof<CompleteByteSlice, bool>,
//     do_parse!(
//         // many_till!(g_le_u16, eof!()) >>
//         count!(g_le_u8, 2000000) >>
//         (true)
//     )
// );

// named!(test_eof_slice<bool>,
//     do_parse!(
//         count!(g_le_u8, 2000000) >>
//         (true)
//     )
// );

// // fn test_eof_long(input: CompleteByteSlice) -> IResult<CompleteByteSlice, CompleteByteSlice> {
// //     many_till!(input, take!(1), eof!())
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_header_ok() {
//         let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
//         let (_, (version, game_type, timestamp)) = parse_header(&buff).unwrap();
//         assert_eq!(version, 20297);
//         assert_eq!(game_type, "COH2_REC");
//         assert_eq!(timestamp, "11/7/2015 1:16 AM");
//     }

//     #[test]
//     fn test_parse_chunky_ok() {
//         let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
//         let (remaining, _) = parse_header(&buff).unwrap();
//         let (_, result) = parse_chunky(remaining).unwrap();
//         assert!(result);
//     }

//     #[test]
//     fn test_parse_eof() {
//         let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
//         let (remaining, _) = test_eof(CompleteByteSlice(&buff)).unwrap();
//     }

//     #[bench]
//     fn bench_parse_eof(b: &mut Bencher) {
//         let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
//         b.iter(|| {
//             test_eof(CompleteByteSlice(&buff)).unwrap()
//             // println!("{:?}", remaining);
//             // println!("{:?}", val);
//         });
//     }

//     #[bench]
//     fn bench_parse_eof_slice(b: &mut Bencher) {
//         let buff = read_into_buffer(Path::new("/Users/ryantaylor/Code/vault/replays/bench.rec"));
//         b.iter(|| {
//             test_eof_slice(&buff).unwrap()
//             // println!("{:?}", remaining);
//         });
//     }

//     fn read_into_buffer(path: &Path) -> Vec<u8> {
//         let mut file = File::open(path).unwrap();
//         let mut buff: Vec<u8> = Vec::new();
//         file.read_to_end(&mut buff).unwrap();
//         buff
//     }
// }
