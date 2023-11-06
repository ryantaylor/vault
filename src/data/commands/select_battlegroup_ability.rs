use crate::data::commands::CommandData;
use crate::data::parser::verify_le_u8;
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct SelectBattlegroupAbility {
    pub player_id: u8,
    pub pgbid: u32,
}

impl SelectBattlegroupAbility {
    #[tracable_parser]
    pub fn parse_command(input: Span) -> ParserResult<CommandData> {
        map(
            tuple((
                take(2u32),
                Self::verify_action_type,
                le_u8,
                take(31u32),
                le_u32,
            )),
            |(_, _, player_id, _, pgbid)| {
                CommandData::SelectBattlegroupAbility(SelectBattlegroupAbility { player_id, pgbid })
            },
        )(input)
    }

    fn verify_action_type(input: Span) -> ParserResult<u8> {
        verify_le_u8(137u8)(input)
    }
}
