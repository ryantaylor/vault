use super::Command;
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::length_value;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug, Copy, Clone)]
pub struct Bundle {
    pub index: u32,
    pub command: Command,
}

impl Bundle {
    #[tracable_parser]
    pub fn parse_bundle(input: Span) -> ParserResult<Bundle> {
        map(
            tuple((le_u32, take(4u32), length_value(le_u32, Command::parse))),
            |(index, _, command)| Bundle { index, command },
        )(input)
    }
}
