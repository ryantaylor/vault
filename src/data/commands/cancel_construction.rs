use crate::data::commands::CommandData;
use crate::data::parser::verify_le_u8;
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::number::complete::{le_u16, le_u8};
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct CancelConstruction {
    pub player_id: u8,
    pub source_identifier: u16,
}

impl CancelConstruction {
    #[tracable_parser]
    pub fn parse_command(input: Span) -> ParserResult<CommandData> {
        map(
            tuple((
                take(2u32),
                Self::verify_action_type,
                le_u8,
                take(26u32),
                le_u16,
            )),
            |(_, _, player_id, _, source_identifier)| {
                CommandData::CancelConstruction(CancelConstruction {
                    player_id,
                    source_identifier,
                })
            },
        )(input)
    }

    fn verify_action_type(input: Span) -> ParserResult<u8> {
        verify_le_u8(56u8)(input)
    }
}
