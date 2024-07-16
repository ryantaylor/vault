use crate::data::chunks::{Chunk, Chunk::DataSdsc, Header};
use crate::data::parser::{parse_utf16_variable, parse_utf8_variable, verify_le_u32};
use crate::data::{ParserResult, Span};
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::{cond, cut, map, map_parser, success};
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct DataSdscChunk {
    _header: Header,
    pub map_file: String,
    pub map_name: String,
    pub map_description: String,
}

impl DataSdscChunk {
    #[tracable_parser]
    pub fn parse(input: Span, header: Header) -> ParserResult<Chunk> {
        cut(map_parser(
            take(header.length),
            map(
                tuple((
                    take(121u32),
                    cond(header.version > 3026, take(8u32)),
                    Self::parse_map_file,
                    Self::parse_map_identifier,
                    alt((verify_le_u32(0u32), success(0u32))),
                    Self::parse_map_identifier,
                )),
                |(_, _, map_file, map_name, _, map_description)| {
                    DataSdsc(DataSdscChunk {
                        _header: header.clone(),
                        map_name,
                        map_file,
                        map_description,
                    })
                },
            ),
        ))(input)
    }

    fn parse_map_file(input: Span) -> ParserResult<String> {
        let (input, (_, section_resources)) = parse_utf8_variable(le_u32)(input)?;
        Ok((input, section_resources))
    }

    fn parse_map_identifier(input: Span) -> ParserResult<String> {
        let (input, (_, section_resources)) = parse_utf16_variable(le_u32)(input)?;
        Ok((input, section_resources))
    }
}
