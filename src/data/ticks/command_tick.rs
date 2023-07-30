use crate::data::parser::verify_le_u32;
use crate::data::ticks::{Bundle, Tick, Tick::Command};
use crate::data::{ParserResult, Span};
use nom::combinator::map;
use nom::multi::{length_count, length_value};
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct CommandTick {
    pub id: u32,
    pub tick_type: u32,
    pub bundles: Vec<Bundle>,
}

impl CommandTick {
    #[tracable_parser]
    pub fn parse_tick(input: Span) -> ParserResult<Tick> {
        map(
            tuple((
                verify_le_u32(0),
                length_value(
                    le_u32,
                    tuple((
                        le_u8,
                        le_u32,
                        le_u32,
                        length_count(le_u32, Bundle::parse_bundle),
                    )),
                ),
            )),
            |(tick_type, (_, id, _, bundles))| {
                Command(CommandTick {
                    id,
                    tick_type,
                    bundles,
                })
            },
        )(input)
    }
}
