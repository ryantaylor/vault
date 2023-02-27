use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::option::Option;
use std::fmt::Debug;
use std::collections::HashMap;
use std::ops::Deref;
use std::thread;
// use test::Bencher;

// use nom::{le_u8, le_u16, le_u32, IResult};
// use nom::types::CompleteByteSlice;

use nom::IResult;
use nom::branch::{alt};
use nom::bytes::complete::{tag, take, take_while};
use nom::combinator::{cut, map, map_res, map_parser, peek, rest_len, verify, cond, eof};
use nom::multi::{count, many0, many1, many_till, length_value, many_m_n};
use nom::number::complete::{le_u8, le_u16, le_u32, le_u64};
use nom::sequence::{preceded, tuple, terminated};
use nom::error::convert_error;
// use nom::dbg_dmp;
// use nom_trace::{Input, tr};
use nom_locate::LocatedSpan;
use nom_tracable::{cumulative_histogram, histogram, tracable_parser, TracableInfo};

use result::Result;
use error::Error;
// use parser::{orig_le_u16, g_le_u8, g_le_u16, g_le_u32, g_le_u64, cbs_le_u16, match_utf8, match_version, match_terminated_utf16};

use parser::{count_n, take_n, parse_utf8_fixed, parse_utf16_terminated, take_zeroes, verify_le_u32, verify_zero_u16, parse_utf8_variable, parse_utf16_variable};

const GAME_TYPE: &'static str = "COH2_REC";
const CHUNKY_NAME: &'static str = "Relic Chunky";

const CHUNKY_TYPE: u32 = 0x1A0A0D;
const CHUNKY_VERSION: u32 = 0x3;

pub type Span<'a> = LocatedSpan<&'a [u8], TracableInfo>;

pub fn parse(path: &Path) -> Option<NomReplay> {
    let mut file = File::open(path).unwrap();
    let mut buff: Vec<u8> = Vec::new();
    file.read_to_end(&mut buff).unwrap();

    if buff.len() == 0 {
        println!("empty file");
        return None;
    }

    let info = TracableInfo::new().parser_width(64).fold("term");
    let input: Span = LocatedSpan::new_extra(&buff, info);

    let result = match parse_replay(input) {
        Ok((_, replay)) => Some(replay),
        Err(e)                  => {
            // print_trace!();
            println!("in error branch");
            // println!("{:#?}", e);
            None
        }
    };

    histogram();
    cumulative_histogram();

    result
}

pub fn parse_directory(path: &Path) -> Result<Vec<NomReplay>, Error> {
    let dir = fs::read_dir(path)?;
    let mut replays: Vec<NomReplay> = Vec::new();
    let mut handles: Vec<_> = Vec::new();

    for item in dir {
        let item = match item {
            Ok(val) => val,
            Err(_) => {
                println!("error reading file in directory");
                continue;
            }
        };

        let path = item.path();
        if path.is_file() {
            match path.extension() {
                Some(ext) => match ext.to_string_lossy().deref() {
                    "rec" => {
                        let path = path.to_owned();
                        let handle = thread::spawn(move || {
                            match parse(&path) {
                                Some(results) => Some(results),
                                None => {
                                    let long_name = path.to_string_lossy();
                                    let long_name = long_name.deref();
                                    println!("Replay failed: {:#?}", long_name);
                                    panic!();
                                }
                            }
                        });
                        handles.push(handle);
                    },
                    _ => println!("skipping {}, not a replay or archive", path.display()),
                },
                None => println!("skipping {}, not a replay or archive", path.display())
            }
        } else if path.is_dir() {
            match parse_directory(&path) {
                Ok(result) => replays.extend(result.into_iter()),
                Err(_) => println!("parse failed"),
            }
        }
    }

    for handle in handles {
        match handle.join() {
            Ok(result) => replays.extend(result.into_iter()),
            Err(_) => println!("parse failed"),
        }
    }

    Ok(replays)
}

#[derive(Debug)]
pub struct NomReplay {
    pub header: Header,
    pub chunkies: Vec<RelicChunky>,
    pub actions: Vec<Box<dyn Action>>
}

#[tracable_parser]
fn parse_replay(input: Span) -> IResult<Span, NomReplay> {
    map(
        tuple((
            parse_header,
            many1(parse_chunky),
            many0(parse_action)
        )),
        |(
            header,
            chunkies,
            actions
        )| {
            NomReplay {
                header,
                chunkies,
                actions
            }
        }
    )(input)
}

#[derive(Debug)]
pub struct Header {
    pub version: u16,
    pub game_type: String,
    pub timestamp: String
}

#[tracable_parser]
fn parse_header(input: Span) -> IResult<Span, Header> {
    map(
        tuple((
            preceded(verify_zero_u16, le_u16),
            parse_utf8_fixed(8usize),
            parse_utf16_terminated,
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

#[derive(Debug)]
pub struct RelicChunky {
    pub name: String,
    pub signature: u32,
    pub major_version: u32,
    pub minor_version: u32, // maybe?
    pub chunks: Vec<Box<dyn Chunk>>
}

#[tracable_parser]
fn parse_chunky(input: Span) -> IResult<Span, RelicChunky> {
    map(
        tuple((
            tag("Relic Chunky"),
            verify_le_u32(0x1A0A0D),
            verify_le_u32(0x4),
            verify_le_u32(0x1),
            many1(parse_chunk)
        )),
        |(
            name,
            signature,
            major_version,
            minor_version,
            chunks
        )| {
            RelicChunky {
                name: String::from_utf8_lossy(name.fragment()).into_owned(),
                signature,
                major_version,
                minor_version,
                chunks
            }
        }
    )(input)
}

pub trait Chunk: Debug + Send {
    fn test(&self) -> String {
        String::from("test")
    }
}

#[derive(Debug, Clone)]
struct ChunkHeader {
    pub chunk_kind: String,
    pub chunk_type: String,
    pub version: u32,
    pub length: u32,
    pub name_length: u32
}

#[tracable_parser]
fn parse_chunk<'a>(input: Span<'a>) -> IResult<Span<'a>, Box<dyn Chunk>> {
    let (input, header) = parse_chunk_header(input)?;

    println!("parsing {}{}", header.chunk_kind, header.chunk_type);

    let parser = match &header.chunk_kind as &str {
        "DATA" => match &header.chunk_type as &str {
            "DATA" => parse_datadata_chunk,
            "SDSC" => parse_datasdsc_chunk,
            "PLAS" => parse_dataplas_chunk,
                 _ => panic!()
        },
        "FOLD" => parse_folder_chunk,
             _ => panic!()
    };

    cut(
        map_parser(
            take(header.length),
            terminated(
                parser(header),
                eof
            )
        )
    )(input)
}

#[derive(Debug)]
struct FOLDChunk {
    pub header: ChunkHeader,
    pub chunks: Vec<Box<dyn Chunk>>
}

impl Chunk for FOLDChunk {}

#[tracable_parser]
fn parse_folder_chunk<'a>(header: ChunkHeader) -> Box<dyn Fn(Span<'a>) -> IResult<Span<'a>, Box<dyn Chunk>>> {
    Box::new(
        move |input| map(
            map_parser(
                take(header.length),
                many0(parse_chunk)
            ),
            |chunks| {
                Box::new(FOLDChunk {
                    header: header.clone(),
                    chunks
                }) as Box<dyn Chunk>
            }
        )(input)
    )
}

#[tracable_parser]
fn parse_datadata_chunk<'a>(header: ChunkHeader) -> Box<dyn Fn(Span<'a>) -> IResult<Span<'a>, Box<dyn Chunk>>> {
    match header.version {
        1 => parse_simple_datadata_chunk(header),
        _ => parse_complex_datadata_chunk(header)
    }
}

#[derive(Debug)]
struct SimpleDATADATAChunk {
    pub header: ChunkHeader,
    pub unknown: Vec<u8>
}

impl Chunk for SimpleDATADATAChunk {}

#[tracable_parser]
fn parse_simple_datadata_chunk<'a>(header: ChunkHeader) -> Box<dyn Fn(Span<'a>) -> IResult<Span<'a>, Box<dyn Chunk>>> {
    Box::new(
        move |input| map(
            take(header.length),
            |unknown: Span| {
                Box::new(SimpleDATADATAChunk {
                    header: header.clone(),
                    unknown: unknown.to_vec()
                }) as Box<dyn Chunk>
            }
        )(input)
    )
}

#[derive(Debug)]
struct ComplexDATADATAChunk {
    pub header: ChunkHeader,
    pub opponent_type: u32,
    pub unknown_flag_1: u32, // 0 or 1
    pub unknown_flag_2: u32, // 0
    pub unknown_flag_3: u16, // 0
    pub rng_seed: u32,
    pub player_count: u32,
    pub player_data: Vec<PlayerData>,
    pub unknown_flag_4: u32, // 0x0 in test
    pub unknown_flag_5: u32, // some large number
    pub unknown_flag_6: u32, // 0x0 in test
    pub unknown_flag_7: u32, // some large number
    pub unknown_flag_8: u32, // 0x0 in test
    pub unknown_flag_9: u32, // 0x4 in test
    pub unknown_flag_10: u32, // 0x0 in test
    pub unknown_flag_11: u32, // 0x0 in test
    pub unknown_flag_12: u32, // 0x1 in test
    pub unknown_flag_13: u32, // 0x0 in test
    pub unknown_flag_14: u32, // 0x2 in test
    pub unknown_flag_15: u32, // 0x1 in test
    pub unknown_flag_16: Option<u32>, // 0x1 in test (exists when version == 28, doesn't when 27)
    pub unknown_text_length: u32, // some long num string (usually zeroes), then colon, then more nums
    pub unknown_text: String,
    pub unknown_flag_17: u16, // 0x1 in test
    pub unknown_flag_18: u32, // 0x1 in test (big endian though)
    pub unknown_flag_19: u32, // 0x0 in test
    pub guid_like_string_length: u32,
    pub guid_like_string: String,
    pub unknown_datadata_block_count: u32, // 0x0 in test
    pub unknown_datadata_blocks: Vec<UnknownDATADATABlock>
}

impl Chunk for ComplexDATADATAChunk {}

#[derive(Debug)]
struct UnknownDATADATABlock {
    pub unknown_id: u32, // seems to usually be <10
    pub unknown_byte_stream: Vec<u8> // 32 bytes of unknown data
}

#[tracable_parser]
fn parse_unknown_datadata_block(input: Span) -> IResult<Span, UnknownDATADATABlock> {
    map(
        tuple::<Span, _, _, _>((
            le_u32,
            take(32usize)
        )),
        |(
            unknown_id,
            unknown_byte_stream
        )| {
            UnknownDATADATABlock {
                unknown_id,
                unknown_byte_stream: unknown_byte_stream.to_vec()
            }
        }
    )(input)
}

#[tracable_parser]
fn parse_complex_datadata_chunk<'a>(header: ChunkHeader) -> Box<dyn Fn(Span<'a>) -> IResult<Span<'a>, Box<dyn Chunk>>> {
    Box::new(
        move |input| map(
            tuple((
                le_u32,
                le_u32,
                le_u32,
                le_u16,
                le_u32,
                count_n(le_u32, parse_player_data),
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                cond(header.version >= 0x1C, le_u32),
                parse_utf8_variable(le_u32),
                tuple((
                    le_u16,
                    le_u32,
                    le_u32,
                    parse_utf8_variable(le_u32),
                    count_n(le_u32, parse_unknown_datadata_block)
                ))
            )),
            |(
                opponent_type,
                unknown_flag_1,
                unknown_flag_2,
                unknown_flag_3,
                rng_seed,
                (player_count, player_data),
                unknown_flag_4,
                unknown_flag_5,
                unknown_flag_6,
                unknown_flag_7,
                unknown_flag_8,
                unknown_flag_9,
                unknown_flag_10,
                unknown_flag_11,
                unknown_flag_12,
                unknown_flag_13,
                unknown_flag_14,
                unknown_flag_15,
                unknown_flag_16,
                (unknown_text_length, unknown_text),
                (
                    unknown_flag_17,
                    unknown_flag_18,
                    unknown_flag_19,
                    (guid_like_string_length, guid_like_string),
                    (unknown_datadata_block_count, unknown_datadata_blocks)
                )
            )| {
                Box::new(ComplexDATADATAChunk {
                    header: header.clone(),
                    opponent_type,
                    unknown_flag_1,
                    unknown_flag_2,
                    unknown_flag_3,
                    rng_seed,
                    player_count,
                    player_data,
                    unknown_flag_4,
                    unknown_flag_5,
                    unknown_flag_6,
                    unknown_flag_7,
                    unknown_flag_8,
                    unknown_flag_9,
                    unknown_flag_10,
                    unknown_flag_11,
                    unknown_flag_12,
                    unknown_flag_13,
                    unknown_flag_14,
                    unknown_flag_15,
                    unknown_flag_16,
                    unknown_text_length,
                    unknown_text,
                    unknown_flag_17,
                    unknown_flag_18,
                    unknown_flag_19,
                    guid_like_string_length,
                    guid_like_string,
                    unknown_datadata_block_count,
                    unknown_datadata_blocks
                }) as Box<dyn Chunk>
            }
        )(input)
    )
}

#[derive(Debug)]
struct PlayerData {
    pub unknown_flag_1: u8, // could be 1 = human player, 0 = cpu player?
    pub name_length: u32,
    pub name: String,
    pub team: u32,
    pub faction_length: u32,
    pub faction: String,
    pub unknown_flag_2: u32, // 5 for army type
    pub unknown_flag_3: u32, // Seb: p00
    pub game_mode_length: u32,
    pub game_mode: String, // Seb: default or skirmish
    pub unknown_flag_4: u32, // Seb: this is not count, it's t1p1 t2p1 t1p2 t2p2 etc
                             // (fixed pos) or I dont even know anymore (for random)
                             // its still count
    pub unknown_flag_5: u32, // something (not position)
    pub unknown_flag_6: u32, // 0x0
    pub unknown_flag_7: u32, // 0x5
    pub unknown_flag_8: u16, // 0x1 - not sure what this is yet
    pub unknown_flag_9: u16, // 0x1 - not sure what this is yet
    pub unknown_flag_10: u64, // u64::MAX if cpu and no steam id, but it will return
                              // 0 in this case so just read anyways
    pub steam_id: u64,
    pub item_block_1_size: u32, // commanders are usually in this block
    pub item_block_2_size: u32, // bulletins are usually in this block
    pub unknown_flag_11: u32, // 0x0
    pub unknown_flag_12: u32, // don't know what this is yet, 2 u32s
    pub unknown_flag_13: u32, // ^
    pub item_data: Vec<Box<dyn ItemData>>
}

#[tracable_parser]
fn parse_player_data(input: Span) -> IResult<Span, PlayerData> {
    map(
        tuple((
            le_u8,
            parse_utf16_variable(le_u32),
            le_u32,
            parse_utf8_variable(le_u32),
            le_u32,
            le_u32,
            parse_utf8_variable(le_u32),
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            le_u16,
            many_m_n(0, 3, preceded(
                peek(verify(le_u16, |n: &u16| *n != 0x1)),
                parse_item_data
            )),
            le_u16,
            le_u64,
            le_u64,
            count(parse_item_data, 3),
            count_n(le_u32, parse_item_data),
            count_n(le_u32, parse_item_data),
            le_u32,
            tuple((
                le_u32,
                le_u32,
            ))
        )),
        |(
            unknown_flag_1,
            (name_length, name),
            team,
            (faction_length, faction),
            unknown_flag_2,
            unknown_flag_3,
            (game_mode_length, game_mode),
            unknown_flag_4,
            unknown_flag_5,
            unknown_flag_6,
            unknown_flag_7,
            unknown_flag_8,
            item_data,
            unknown_flag_9,
            unknown_flag_10,
            steam_id,
            other_item_data,
            (item_block_1_size, item_block_1),
            (item_block_2_size, item_block_2),
            unknown_flag_11,
            (
                unknown_flag_12,
                unknown_flag_13
            )
        )| {
            let items = vec![item_data, other_item_data, item_block_1, item_block_2];

            PlayerData {
                unknown_flag_1,
                name_length,
                name,
                team,
                faction_length,
                faction,
                unknown_flag_2,
                unknown_flag_3,
                game_mode_length,
                game_mode,
                unknown_flag_4,
                unknown_flag_5,
                unknown_flag_6,
                unknown_flag_7,
                unknown_flag_8,
                unknown_flag_9,
                unknown_flag_10,
                steam_id,
                item_block_1_size,
                item_block_2_size,
                unknown_flag_11,
                unknown_flag_12,
                unknown_flag_13,
                item_data: items.into_iter().flatten().collect()
            }
        }
    )(input)
}

pub trait ItemData: Debug + Send {
    fn test(&self) -> String {
        String::from("test")
    }
}

#[tracable_parser]
fn parse_item_data(input: Span) -> IResult<Span, Box<dyn ItemData>> {
    alt((
        parse_player_item_data,
        parse_special_player_item_data,
        parse_cpu_item_data,
        parse_empty_item_data
    ))(input)
}

#[derive(Debug)]
struct PlayerItemData {
    pub item_type: u16,
    pub selection_id: u32,
    pub unknown_flag_1: u32, // 0x0
    pub server_id: u32,
    pub unknown_flag_2: u32, // 0x0
    pub remaining_buffer_size: u16,
    pub remaining_buffer: Vec<u8>
}

impl ItemData for PlayerItemData {}

#[tracable_parser]
fn parse_player_item_data(input: Span) -> IResult<Span, Box<dyn ItemData>> {
    map(
        tuple::<Span, _, _, _>((
            verify(le_u16, |n: &u16| *n == 0x109),
            le_u32,
            le_u32,
            le_u32,
            le_u32,
            take_n(le_u16)
        )),
        |(
            item_type,
            selection_id,
            unknown_flag_1,
            server_id,
            unknown_flag_2,
            (remaining_buffer_size, remaining_buffer)
        )| {
            Box::new(PlayerItemData {
                item_type,
                selection_id,
                unknown_flag_1,
                server_id,
                unknown_flag_2,
                remaining_buffer_size,
                remaining_buffer: remaining_buffer.to_vec()
            }) as Box<dyn ItemData>
        }
    )(input)
}

#[derive(Debug)]
struct SpecialPlayerItemData {
    pub item_type: u16,
    pub unknown_data: Vec<u8>, // lots of data, no idea what it is
    pub unknown_flag_1: u32, // something to do with custom decals
    pub unknown_flag_2: u8 // not sure, was 0x40 in test replay
}

impl ItemData for SpecialPlayerItemData {}

#[tracable_parser]
fn parse_special_player_item_data(input: Span) -> IResult<Span, Box<dyn ItemData>> {
    map(
        tuple::<Span, _, _, _>((
            verify(le_u16, |n: &u16| *n == 0x216),
            take(16usize),
            le_u32,
            le_u8
        )),
        |(
            item_type,
            unknown_data,
            unknown_flag_1,
            unknown_flag_2
        )| {
            Box::new(SpecialPlayerItemData {
                item_type,
                unknown_data: unknown_data.to_vec(),
                unknown_flag_1,
                unknown_flag_2
            }) as Box<dyn ItemData>
        }
    )(input)
}

#[derive(Debug)]
struct CPUItemData {
    pub item_type: u16,
    pub unknown_flag_1: u8, // 0x1
    pub unknown_flag_2: u32 // gotta figure out what this is
}

impl ItemData for CPUItemData {}

#[tracable_parser]
fn parse_cpu_item_data(input: Span) -> IResult<Span, Box<dyn ItemData>> {
    map(
        tuple((
            verify(le_u16, |n: &u16| *n == 0x206),
            le_u8,
            le_u32
        )),
        |(
            item_type,
            unknown_flag_1,
            unknown_flag_2
        )| {
            Box::new(CPUItemData {
                item_type,
                unknown_flag_1,
                unknown_flag_2
            }) as Box<dyn ItemData>
        }
    )(input)
}

#[derive(Debug)]
struct EmptyItemData {
    pub item_type: u16
}

impl ItemData for EmptyItemData {}

#[tracable_parser]
fn parse_empty_item_data(input: Span) -> IResult<Span, Box<dyn ItemData>> {
    map(
        verify(le_u16, |n: &u16| *n == 0x1),
        |item_type| {
            Box::new(EmptyItemData {
                item_type
            }) as Box<dyn ItemData>
        }
    )(input)
}

#[derive(Debug)]
struct DATASDSCChunk {
    pub header: ChunkHeader,
    pub unknown_flag_1: u32, // 0x0
    pub unknown_flag_2: u32, // 0x0
    pub unknown_flag_3: u32, // can be 1 or 2?
    pub unknown_flag_4: u32, // 0x3
    pub unknown_flag_5: u32, // 0x0
    pub unknown_flag_6: u32, // 0x0
    pub unknown_flag_7: u32, // 0x0
    pub map_file_length: u32,
    pub map_file: String,
    pub unknown_data_1: Vec<u8>, // something to do with map start positions?
    pub map_name_length: u32,
    pub map_name: String,
    pub map_name_localized_length: u32,
    pub map_name_localized: String,
    pub map_description_length: u32,
    pub map_description: String,
    pub map_players: u32,
    pub map_width: u32,
    pub map_height: u32,
    pub audio_data_length: u32,
    pub audio_data: String,
    pub map_long_name_length: u32,
    pub map_long_name: String,
    pub unknown_data_2: Vec<u8>,
    pub map_sub_file_length: u32,
    pub map_sub_file: String, // this was DATA:scenarios\mp\2p_divide\2p_divide in a ToW rec
    pub map_sub_name_length: u32,
    pub map_sub_name: String, // this was 2p_divide in a ToW rec
    pub map_sub_sub_file_length: u32,
    pub map_sub_sub_file: String, // this was DATA:scenarios\mp\2p_divide in that ToW rec
    pub environment_data_count: u32,
    pub environment_data: Vec<EnvironmentData>,
    pub unknown_flag_8: u32, // 0x2?
    pub unknown_data_3: Vec<u8>,
    pub season_length: u32,
    pub season: String, // usually only when set to winter
    pub unknown_byte: u8, // 0x1 in a test, but not sure really
    pub unknown_flag_9: u32, // 0x4?
    pub campaign_description_length: u32,
    pub campaign_description: String,
    pub unknown_length_maybe: u32,
    pub unknown_id_maybe: u32,
    pub unknown_data_5_length: u32, // long num string (usually zeroes), then colon, then more nums
    pub unknown_data_5: String,
    pub unknown_flag_10: u32, // 0x0
    pub icon_data_block_1_length: u32,
    pub icon_data_block_1: Vec<IconData>,
    pub icon_data_block_2_length: u32,
    pub icon_data_block_2: Vec<IconData>,
    pub icon_data_block_3_length: u32,
    pub icon_data_block_3: Vec<IconData>,
    pub location_data: Option<LocationData> // only present in version 2020 (and maybe above)
}

impl Chunk for DATASDSCChunk {}

#[derive(Debug)]
struct IconData {
    pub unknown_flag_1: u32, // maybe some ID?
    pub unknown_flag_2: u32, // ^
    pub icon_length: u32,
    pub icon: String
}

#[derive(Debug)]
struct EnvironmentData {
    pub unknown_id: u32, // at least I think it's an ID
    pub filename_length: u32,
    pub filename: String
}

// Only present in version 2020+ of SDSC chunk (not 2019)
#[derive(Debug)]
struct LocationData {
    pub count: u32, // 0x1 maybe
    pub data: Vec<(u32, String)> // (location_length, location)
}

#[tracable_parser]
fn parse_location_data(input: Span) -> IResult<Span, LocationData> {
    map(
        count_n(le_u32, parse_utf8_variable(le_u32))
        , |(count, data)| {
            LocationData {
                count,
                data
            }
        }
    )(input)
}

#[tracable_parser]
fn parse_environment_data(input: Span) -> IResult<Span, EnvironmentData> {
    map(
        tuple((
            le_u32,
            parse_utf8_variable(le_u32)
        )),
        |(
            unknown_id,
            (filename_length, filename)
        )| {
            EnvironmentData {
                unknown_id,
                filename_length,
                filename
            }
        }
    )(input)
}

#[tracable_parser]
fn parse_datasdsc_chunk<'a>(header: ChunkHeader) -> Box<dyn Fn(Span<'a>) -> IResult<Span<'a>, Box<dyn Chunk>>> {
    Box::new(
        move |input| map(
            tuple((
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                parse_utf8_variable(le_u32),
                take(16usize),
                parse_utf16_variable(le_u32),
                parse_utf16_variable(le_u32),
                parse_utf16_variable(le_u32),
                le_u32,
                le_u32,
                le_u32,
                parse_utf8_variable(le_u32),
                parse_utf16_variable(le_u32),
                take(16usize),
                parse_utf8_variable(le_u32),
                parse_utf8_variable(le_u32),
                tuple((
                    parse_utf8_variable(le_u32),
                    count_n(le_u32, parse_environment_data),
                    le_u32,
                    take(13usize),
                    parse_utf8_variable(le_u32),
                    le_u8,
                    le_u32,
                    parse_utf16_variable(le_u32),
                    le_u32,
                    le_u32,
                    parse_utf8_variable(le_u32),
                    le_u32,
                    count_n(le_u32, parse_icon_data),
                    count_n(le_u32, parse_icon_data),
                    count_n(le_u32, parse_icon_data),
                    cond(header.version >= 0x7E4, parse_location_data)
                ))
            )),
            |(
                unknown_flag_1,
                unknown_flag_2,
                unknown_flag_3,
                unknown_flag_4,
                unknown_flag_5,
                unknown_flag_6,
                unknown_flag_7,
                (map_file_length, map_file),
                unknown_data_1,
                (map_name_length, map_name),
                (map_name_localized_length, map_name_localized),
                (map_description_length, map_description),
                map_players,
                map_width,
                map_height,
                (audio_data_length, audio_data),
                (map_long_name_length, map_long_name),
                unknown_data_2,
                (map_sub_file_length, map_sub_file),
                (map_sub_name_length, map_sub_name),
                (
                    (map_sub_sub_file_length, map_sub_sub_file),
                    (environment_data_count, environment_data),
                    unknown_flag_8,
                    unknown_data_3,
                    (season_length, season),
                    unknown_byte,
                    unknown_flag_9,
                    (campaign_description_length, campaign_description),
                    unknown_length_maybe,
                    unknown_id_maybe,
                    (unknown_data_5_length, unknown_data_5),
                    unknown_flag_10,
                    (icon_data_block_1_length, icon_data_block_1),
                    (icon_data_block_2_length, icon_data_block_2),
                    (icon_data_block_3_length, icon_data_block_3),
                    location_data
                )
            )| {
                Box::new(DATASDSCChunk {
                    header: header.clone(),
                    unknown_flag_1,
                    unknown_flag_2,
                    unknown_flag_3,
                    unknown_flag_4,
                    unknown_flag_5,
                    unknown_flag_6,
                    unknown_flag_7,
                    map_file_length,
                    map_file,
                    unknown_data_1: unknown_data_1.to_vec(),
                    map_name_length,
                    map_name,
                    map_name_localized_length,
                    map_name_localized,
                    map_description_length,
                    map_description,
                    map_players,
                    map_width,
                    map_height,
                    audio_data_length,
                    audio_data,
                    map_long_name_length,
                    map_long_name,
                    unknown_data_2: unknown_data_2.to_vec(),
                    map_sub_file_length,
                    map_sub_file,
                    map_sub_name_length,
                    map_sub_name,
                    map_sub_sub_file_length,
                    map_sub_sub_file,
                    environment_data_count,
                    environment_data,
                    unknown_flag_8,
                    unknown_data_3: unknown_data_3.to_vec(),
                    season_length,
                    season,
                    unknown_byte,
                    unknown_flag_9,
                    campaign_description_length,
                    campaign_description,
                    unknown_length_maybe,
                    unknown_id_maybe,
                    unknown_data_5_length,
                    unknown_data_5,
                    unknown_flag_10,
                    icon_data_block_1_length,
                    icon_data_block_1,
                    icon_data_block_2_length,
                    icon_data_block_2,
                    icon_data_block_3_length,
                    icon_data_block_3,
                    location_data
                }) as Box<dyn Chunk>
            }
        )(input)
    )
}

#[tracable_parser]
fn parse_icon_data(input: Span) -> IResult<Span, IconData> {
    map(
        tuple((
            le_u32,
            le_u32,
            parse_utf8_variable(le_u32)
        )),
        |(
            unknown_flag_1,
            unknown_flag_2,
            (icon_length, icon)
        )| {
            IconData {
                unknown_flag_1,
                unknown_flag_2,
                icon_length,
                icon
            }
        }
    )(input)
}

#[derive(Debug)]
struct DATAPLASChunk {
    pub header: ChunkHeader,
    pub unknown_data_length: u32, // was 8 on a test 4v4 replay
    pub unknown_data: Vec<u32>    // seems like some ids, maybe positions? player ids?
                                  // after more testing, looks like positions. n u32s where
                                  // n equals the number of players in the game, and one of
                                  // the integers appears in every command action block.
}

impl Chunk for DATAPLASChunk {}

#[tracable_parser]
fn parse_dataplas_chunk<'a>(header: ChunkHeader) -> Box<dyn Fn(Span<'a>) -> IResult<Span<'a>, Box<dyn Chunk>>> {
    Box::new(
        move |input| map(
            count_n(le_u32, le_u32),
            |(unknown_data_length, unknown_data)| {
                Box::new(DATAPLASChunk {
                    header: header.clone(),
                    unknown_data_length,
                    unknown_data: unknown_data.to_vec()
                }) as Box<dyn Chunk>
            }
        )(input)
    )
}

// struct ChunkHeader {
//     pub chunk_kind: String,
//     pub chunk_type: String,
//     pub version: u32,
//     pub length: u32,
//     pub name_length: u32,
//     pub min_version: u32, // according to Copernicus
//     pub flags: u32 // according to Copernicus
// }

#[tracable_parser]
fn parse_chunk_header(input: Span) -> IResult<Span, ChunkHeader> {
    map(
        tuple((
            alt((
                tag("DATA"),
                tag("FOLD")
            )),
            cut(
                tuple((
                    parse_utf8_fixed(4usize),
                    le_u32,
                    le_u32,
                    le_u32
                ))
            )
        )),
        |(
            chunk_kind,
            (
                chunk_type,
                version,
                length,
                name_length
            )
        )| {
            ChunkHeader {
                chunk_kind: String::from_utf8_lossy(chunk_kind.fragment()).into_owned(),
                chunk_type: chunk_type.to_owned(),
                version,
                length,
                name_length
            }
        }
    )(input)
}

pub trait Action: Debug + Send {
    fn test(&self) -> String {
        String::from("test")
    }
}

#[tracable_parser]
fn parse_action(input: Span) -> IResult<Span, Box<dyn Action>> {
    alt((
        parse_tick,
        parse_populated_chat_action,
        parse_empty_chat_action
    ))(input)
}

#[derive(Debug)]
struct Tick {
    pub action_type: u32, // 0x0 for command ticks
    pub length: u32,
    pub unknown_flag_1: u8, // usually 0x20 but can be 0x0
    pub tick_id: u32,
    pub unknown_flag_2: u32, // some id
    pub bundle_count: u32,
    pub bundles: Vec<Bundle>
}

impl Action for Tick {}

#[tracable_parser]
fn parse_tick(input: Span) -> IResult<Span, Box<dyn Action>> {
    map(
        tuple((
            verify(le_u32, |n: &u32| *n == 0),
            peek(le_u32),
            length_value(
                le_u32,
                tuple((
                    le_u8,
                    le_u32,
                    le_u32,
                    count_n(le_u32, parse_bundle)
                ))
            )
        )),
        |(
            action_type,
            length,
            (
                unknown_flag_1,
                tick_id,
                unknown_flag_2,
                (bundle_count, bundles)
            )
        )| {
            Box::new(Tick {
                action_type,
                length,
                unknown_flag_1,
                tick_id,
                unknown_flag_2,
                bundle_count,
                bundles
            }) as Box<dyn Action>
        }
    )(input)
}

#[derive(Debug)]
struct Bundle {
    pub unknown_flag_1: u32, // maybe bundle part count?
    pub unknown_flag_2: u32, // Seb: thought 0 but can be 33554432
    pub length: u32,
    pub checksum: u8, // checksum == bundle_length % 256 else error
    pub commands: Vec<Command>
}

#[tracable_parser]
fn parse_bundle(input: Span) -> IResult<Span, Bundle> {
    let (input, (
        unknown_flag_1,
        unknown_flag_2,
        length,
        checksum
    )) = tuple((
        le_u32,
        le_u32,
        le_u32,
        le_u8
    ))(input)?;

    let (input, data) = take(length)(input)?;

    let (_, commands) = many0(parse_command)(data)?;

    Ok((input,
        Bundle {
            unknown_flag_1,
            unknown_flag_2,
            length,
            checksum,
            commands
    }))
}

#[derive(Debug)]
struct Command {
    pub data_length: u8,
    pub unknown_flag_1: u8, // not sure? mostly 0 I think
    pub action_type: u8,
    pub unknown_flag_2: u8, // base location?
    pub unknown_flag_3: u8, // part of player ID?
    pub player_id: u8,
    pub plas_flag: u16, // matches an ID in the DATAPLAS chunk
    pub unknown_flag_5: u16, // lots of 0, 16 then 20546 2054720802 21085
    pub unknown_flag_6: u8, // command type (CMD, PCMD, SCMD)
    pub unknown_flag_7: u8, // some sort of target ID (unit/building/player)
    pub command_sub_id: u8,
    pub command_data: Vec<u8>
}

#[tracable_parser]
fn parse_command(input: Span) -> IResult<Span, Command> {
    map(
        length_value(
            peek::<Span, _, _, _>(le_u8),
            tuple((
                le_u8,
                le_u8,
                le_u8,
                le_u8,
                le_u8,
                le_u8,
                le_u16,
                le_u16,
                le_u8,
                le_u8,
                le_u8,
                take_while(|_| true)
            ))
        ),
        |(
            data_length,
            unknown_flag_1,
            action_type,
            unknown_flag_2,
            unknown_flag_3,
            player_id,
            plas_flag,
            unknown_flag_5,
            unknown_flag_6,
            unknown_flag_7,
            command_sub_id,
            command_data
        )| {
            Command {
                data_length,
                unknown_flag_1,
                action_type,
                unknown_flag_2,
                unknown_flag_3,
                player_id,
                plas_flag,
                unknown_flag_5,
                unknown_flag_6,
                unknown_flag_7,
                command_sub_id,
                command_data: command_data.to_vec()
            }
        }
    )(input)
}

pub trait ChatAction: Debug + Send {
    fn test(&self) -> String {
        String::from("test")
    }
}

impl<T> Action for T where T: ChatAction {}

#[derive(Debug)]
struct PopulatedChatAction {
    pub action_type: u32, // 0x1 for chat actions it seems
    pub length: u32,
    pub chat_flag: u32, // Seb: is chat? most 1 few 0 (maybe number of messages in action?)
    pub unknown_flag_1: u32, // length
    pub unknown_flag_2: u32, // Seb: chat nbr 2 6 or few 4
    pub name_length: u32,
    pub name: String,
    pub message_length: u32,
    pub message: String,
    pub unknown_data_length: u32, // not sure what this is
    pub unknown_data: Vec<u16>    // some numeric ids? all u16s
}

impl ChatAction for PopulatedChatAction {}

#[tracable_parser]
fn parse_populated_chat_action(input: Span) -> IResult<Span, Box<dyn Action>> {
    map(
        tuple((
            verify(le_u32, |n: &u32| *n == 1),
            peek(le_u32),
            length_value(
                le_u32,
                tuple((
                    verify(le_u32, |n: &u32| *n == 1),
                    le_u32,
                    le_u32,
                    parse_utf16_variable(le_u32),
                    parse_utf16_variable(le_u32),
                    count_n(le_u32, le_u16)
                ))
            )
        )),
        |(
            action_type,
            length,
            (
                chat_flag,
                unknown_flag_1,
                unknown_flag_2,
                (name_length, name),
                (message_length, message),
                (unknown_data_length, unknown_data)
            )
        )| {
            Box::new(PopulatedChatAction {
                action_type,
                length,
                chat_flag,
                unknown_flag_1,
                unknown_flag_2,
                name_length,
                name,
                message_length,
                message,
                unknown_data_length,
                unknown_data
            }) as Box<dyn Action>
        }
    )(input)
}

#[derive(Debug)]
struct EmptyChatAction {
    pub action_type: u32, // 0x1 for chat actions it seems
    pub length: u32,
    pub chat_flag: u32, // Seb: is chat? most 1 few 0 (maybe number of messages in action?)
    pub unknown_flag_1: u32, // 0x8
    pub unknown_flag_2: u32, // Seb: special E9 03 00 00 1000 to 1006
    pub unknown_flag_3: u32 // 0x0
}

impl ChatAction for EmptyChatAction {}

#[tracable_parser]
fn parse_empty_chat_action(input: Span) -> IResult<Span, Box<dyn Action>> {
    map(
        tuple((
            verify(le_u32, |n: &u32| *n == 1),
            peek(le_u32),
            length_value(
                le_u32,
                tuple((
                    verify(le_u32, |n: &u32| *n == 0),
                    le_u32,
                    le_u32,
                    le_u32
                ))
            )
        )),
        |(
            action_type,
            length,
            (
                chat_flag,
                unknown_flag_1,
                unknown_flag_2,
                unknown_flag_3
            )
        )| {
            Box::new(EmptyChatAction {
                action_type,
                length,
                chat_flag,
                unknown_flag_1,
                unknown_flag_2,
                unknown_flag_3
            }) as Box<dyn Action>
        }
    )(input)
}

// fn parse_action(input: &[u8]) -> IResult<&[u8], Vec<Box<dyn Action>>> {

// }

// fn parse_actions(input: &[u8]) -> IResult<&[u8], Box<dyn Action>> {
//     map(
//         alt((

//         ))
//     )
// }

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



// trait Foo {}

// struct Bar {
//     pub id: i32
// }

// impl Foo for Bar {}

// struct Baz {
//     pub id: i32
// }

// impl Foo for Baz {}

// fn bar_closure() -> Box<dyn Fn() -> Box<dyn Foo>> {
//     Box::new(move || {
//         Box::new(Bar { id: 1 })
//     })
// }

// fn baz_closure() -> Box<dyn Fn() -> Box<dyn Foo>> {
//     Box::new(move || {
//         Box::new(Baz { id: 1 })
//     })
// }

// fn foo_match() {
//     let a = 1;
//     let foo_fn = match a {
//         1 => bar_closure,
//         _ => baz_closure
//     };
// }
