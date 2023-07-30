use crate::data::commands::CommandData;
use crate::data::commands::CommandData::{BuildSquadData, UnknownCommandData};
use crate::data::parser::{verify_le_u32, verify_le_u8};
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{flat_map, map, map_parser};
use nom::multi::length_value;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct Unknown {
    pub action_type: u8,
    pub base_location: u8,
    pub player_id: u8,
}

impl Unknown {
    #[tracable_parser]
    pub fn parse_command(input: Span) -> ParserResult<CommandData> {
        map(
            tuple((take(2u32), le_u8, le_u8, take(1u32), le_u8)),
            |(_, action_type, base_location, _, player_id)| {
                UnknownCommandData(Unknown {
                    action_type,
                    base_location,
                    player_id,
                })
            },
        )(input)
    }
}
