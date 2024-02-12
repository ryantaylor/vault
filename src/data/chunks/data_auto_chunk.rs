use crate::data::chunks::header::Header;
use crate::data::chunks::Chunk;
use crate::data::chunks::Chunk::DataAuto;
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{cut, map, map_parser};
use nom::number::complete::le_u8;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct DataAutoChunk {
    pub automatch: bool,
}

impl DataAutoChunk {
    #[tracable_parser]
    pub fn parse(input: Span, header: Header) -> ParserResult<Chunk> {
        cut(map_parser(
            take(header.length),
            map(le_u8, |automatch| {
                DataAuto(DataAutoChunk {
                    automatch: automatch == 1,
                })
            }),
        ))(input)
    }
}
