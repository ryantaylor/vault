use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::option::Option;
use std::fmt::Debug;
use std::collections::HashMap;
// use test::Bencher;

// use nom::{le_u8, le_u16, le_u32, IResult};
// use nom::types::CompleteByteSlice;

use nom::IResult;
use nom::branch::{alt};
use nom::bytes::complete::{tag, take, take_while};
use nom::combinator::{cut, map, map_res, map_parser, peek, rest_len, verify};
use nom::multi::{count, many0, many_till, length_value};
use nom::number::complete::{le_u8, le_u16, le_u32, le_u64};
use nom::sequence::{preceded, tuple, terminated};
use nom::error::convert_error;
use nom::dbg_dmp;
use nom_trace::{Input, tr};

// use parser::{orig_le_u16, g_le_u8, g_le_u16, g_le_u32, g_le_u64, cbs_le_u16, match_utf8, match_version, match_terminated_utf16};

use parser::{enforce_end_of_input, count_n, take_n, parse_utf8_fixed, parse_utf16_terminated, take_zeroes, verify_le_u32, verify_zero_u16, parse_utf8_variable, parse_utf16_variable};

fn t<I,O,E,F>(name: &'static str, f: F) -> impl Fn(I) -> IResult<I,O,E>
  where Input: From<I>,
        F: Fn(I) -> IResult<I,O,E>,
        I: Clone,
        O: Debug,
        E: Debug {
  tr("default", name, f)
}

#[derive(Debug)]
struct Replay {
    pub header: Header,
    pub chunkies: Vec<Chunky>,
    pub actions: Vec<Action>
}

#[derive(Debug)]
struct Header<'a> {
    pub check_byte: u16,
    pub version: u16,
    pub game_type: String,
    pub timestamp: String,
    pub data: &'a [u8]
}

#[derive(Debug)]
struct Chunky {
    pub name: String,
    pub signature: u32,
    pub major_version: u32,
    pub minor_version: u32, // maybe?
    pub chunk_offset: u32, // bytes from start of chunky to start of first member chunk
    pub unknown_offset: u32, // usually 0x1C
    pub unknown_id: u32, // usually 0x1
    pub chunks: Vec<Chunk>
}

struct Chunk {
    
}
