use crate::data::chunks::{Chunk, Chunk::DataData, Header, TrashDataChunk};
use crate::data::parser::parse_utf8_variable;
use crate::data::{ParserResult, Player, Span};
use byteorder::{LittleEndian, ReadBytesExt};
use nom::bytes::complete::{tag, take, take_while};
use nom::character::{is_digit, is_hex_digit};
use nom::combinator::{cut, map, map_parser, peek, verify};
use nom::multi::{length_count, length_data, length_value, many_m_n, many_till};
use nom::number::complete::{le_u32, le_u64};
use nom::sequence::{separated_pair, tuple};
use nom_tracable::tracable_parser;
use uuid::Uuid;

#[derive(Debug)]
pub struct Option {
    _name: String,
    _value: u32,
}

impl Option {
    #[tracable_parser]
    pub fn parse_option(input: Span) -> ParserResult<Option> {
        map(
            tuple((parse_utf8_variable(le_u32), le_u32)),
            |((_, name), value)| Option {
                _name: name,
                _value: value,
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct DataDataChunk {
    _header: Header,
    _opponent_type: u32,
    pub players: Vec<Player>,
    pub skirmish: bool,
    pub matchhistory_id: u64,
    _options: Vec<Option>,
    pub mod_uuid: Uuid,
    _unknown_number: u32,
}

impl DataDataChunk {
    #[tracable_parser]
    pub fn parse(input: Span, header: Header) -> ParserResult<Chunk> {
        if header.version == 1 {
            return TrashDataChunk::parse(input, header);
        }

        cut(map_parser(
            take(header.length),
            map(
                tuple((
                    Self::parse_opponent_type,
                    take(6u32),
                    Self::parse_players,
                    length_data(le_u32),
                    Self::parse_skirmish_flag,
                    le_u64,
                    take(24u32),
                    Self::parse_options,
                    take(12u32),
                    Self::parse_mod_info,
                )),
                |(
                    opponent_type,
                    _,
                    players,
                    _,
                    skirmish,
                    matchhistory_id,
                    _,
                    options,
                    _,
                    (mod_uuid, unknown_number),
                )| {
                    DataData(DataDataChunk {
                        _header: header.clone(),
                        _opponent_type: opponent_type,
                        players,
                        skirmish,
                        matchhistory_id,
                        _options: options,
                        mod_uuid,
                        _unknown_number: unknown_number,
                    })
                },
            ),
        ))(input)
    }

    #[tracable_parser]
    fn parse_opponent_type(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }

    fn parse_players(input: Span) -> ParserResult<Vec<Player>> {
        length_count(le_u32, Player::parse_player)(input)
    }

    #[tracable_parser]
    fn parse_options(input: Span) -> ParserResult<Vec<Option>> {
        map(
            many_till(
                Option::parse_option,
                map_parser(
                    peek(take(12u32)),
                    many_m_n(3, 3, verify(le_u32, |n| *n == 0 || *n == 1)),
                ),
            ),
            |(options, _)| options,
        )(input)
    }

    #[tracable_parser]
    fn parse_skirmish_flag(input: Span) -> ParserResult<bool> {
        map(parse_utf8_variable(le_u32), |(_, id)| !id.is_empty())(input)
    }

    #[tracable_parser]
    fn parse_mod_info(input: Span) -> ParserResult<(Uuid, u32)> {
        length_value(
            le_u32,
            map(
                separated_pair(take_while(is_hex_digit), tag(":"), take_while(is_digit)),
                |(mod_uuid, unknown_number): (Span, Span)| {
                    (
                        Uuid::try_parse_ascii(&mod_uuid).unwrap(),
                        unknown_number
                            .into_fragment()
                            .read_u32::<LittleEndian>()
                            .unwrap(),
                    )
                },
            ),
        )(input)
    }
}
