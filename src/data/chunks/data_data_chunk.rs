use crate::data::chunks::{Chunk, Chunk::DataData, Header, TrashDataChunk};
use crate::data::parser::parse_utf8_variable;
use crate::data::{ParserResult, Player, Span};
use nom::bytes::complete::take;
use nom::combinator::{cut, flat_map, map, map_parser};
use nom::multi::{length_count, length_data};
use nom::number::complete::{le_u32, le_u64};
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct DataDataChunk {
    pub header: Header,
    pub opponent_type: u32,
    pub players: Vec<Player>,
    pub matchhistory_id: u64,
    pub section_resources: String,
    pub option_resources: String,
    pub section_tickets: String,
    pub option_tickets: String,
    pub unknown_string: String,
}

impl DataDataChunk {
    #[tracable_parser]
    pub fn parse(input: Span, header: Header, version: u16) -> ParserResult<Chunk> {
        if header.version == 1 {
            return TrashDataChunk::parse(input, header);
        }

        cut(map_parser(
            take(header.length),
            map(
                tuple((
                    Self::parse_opponent_type,
                    take(6u32),
                    Self::parse_players(version),
                    length_data(le_u32),
                    take(4u32),
                    le_u64,
                    take(4u32),
                    take(20u32),
                    Self::parse_resource_string,
                    take(4u32),
                    Self::parse_resource_string,
                    take(4u32),
                    Self::parse_resource_string,
                    take(4u32),
                    Self::parse_resource_string,
                    take(16u32),
                    Self::parse_resource_string,
                )),
                |(
                    opponent_type,
                    _,
                    players,
                    _,
                    _,
                    matchhistory_id,
                    _,
                    _,
                    section_resources,
                    _,
                    option_resources,
                    _,
                    section_tickets,
                    _,
                    option_tickets,
                    _,
                    unknown_string,
                )| {
                    DataData(DataDataChunk {
                        header: header.clone(),
                        opponent_type,
                        players,
                        matchhistory_id,
                        section_resources,
                        option_resources,
                        section_tickets,
                        option_tickets,
                        unknown_string,
                    })
                },
            ),
        ))(input)
    }

    #[tracable_parser]
    fn parse_opponent_type(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }

    fn parse_players(version: u16) -> impl FnMut(Span) -> ParserResult<Vec<Player>> {
        move |input: Span| length_count(le_u32, Player::parse_player(version))(input)
    }

    fn parse_resource_string(input: Span) -> ParserResult<String> {
        let (input, (_, section_resources)) = parse_utf8_variable(le_u32)(input)?;
        Ok((input, section_resources))
    }
}
