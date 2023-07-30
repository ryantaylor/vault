use crate::data::commands::CommandData;
use crate::data::commands::CommandData::BuildSquadData;
use crate::data::parser::{verify_le_u32, verify_le_u8};
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{flat_map, map, map_parser};
use nom::multi::length_value;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct BuildSquad {
    pub base_location: u8,
    pub player_id: u8,
    pub pgbid: u32,
}

impl BuildSquad {
    #[tracable_parser]
    pub fn parse_command(input: Span) -> ParserResult<CommandData> {
        map(
            tuple((
                take(2u32),
                Self::verify_action_type,
                le_u8,
                take(1u32),
                le_u8,
                take(29u32),
                le_u32,
            )),
            |(_, _, base_location, _, player_id, _, pgbid)| {
                BuildSquadData(BuildSquad {
                    base_location,
                    player_id,
                    pgbid,
                })
            },
        )(input)
    }

    fn verify_action_type(input: Span) -> ParserResult<u8> {
        verify_le_u8(0x3)(input)
    }
}
