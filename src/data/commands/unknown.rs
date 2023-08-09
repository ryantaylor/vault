use crate::data::commands::CommandData;
use crate::data::commands::CommandData::UnknownCommandData;
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::number::complete::le_u8;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct Unknown {
    pub action_type: u8,
    pub player_id: u8,
}

impl Unknown {
    #[tracable_parser]
    pub fn parse_command(input: Span) -> ParserResult<CommandData> {
        map(
            tuple((take(2u32), le_u8, le_u8)),
            |(_, action_type, player_id)| {
                UnknownCommandData(Unknown {
                    action_type,
                    player_id,
                })
            },
        )(input)
    }
}
