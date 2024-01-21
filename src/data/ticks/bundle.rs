use crate::data::commands::{CommandData, Raw};
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{map, peek};
use nom::multi::length_value;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct Bundle {
    pub index: u32,
    pub command: CommandData,
    pub raw: Raw,
}

impl Bundle {
    #[tracable_parser]
    pub fn parse_bundle(input: Span) -> ParserResult<Bundle> {
        map(
            tuple((
                le_u32,
                take(4u32),
                peek(length_value(le_u32, Raw::parse_command)),
                length_value(le_u32, CommandData::parse)
            )),
            |(index, _, raw, command)| Bundle { index, command, raw },
        )(input)
    }
}
