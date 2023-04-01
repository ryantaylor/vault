use crate::data::parser::verify_le_u32;
use crate::data::ticks::{Tick, Tick::Command};
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{flat_map, map, map_parser};
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;

#[derive(Debug)]
pub struct CommandTick {
    pub id: u32,
    pub tick_type: u32,
}

impl CommandTick {
    pub fn parse_tick(input: Span) -> ParserResult<Tick> {
        map(
            tuple((
                verify_le_u32(0),
                map_parser(flat_map(le_u32, take), tuple((le_u8, le_u32, le_u32))),
            )),
            |(tick_type, (_, id, _))| Command(CommandTick { id, tick_type }),
        )(input)
    }
}
