use data::parser::parse_utf8_fixed;
use data::{ParserResult, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, map};
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug, Clone)]
pub struct Header {
    pub chunk_kind: String,
    pub chunk_type: String,
    pub version: u32,
    pub length: u32,
    pub name_length: u32,
}

impl Header {
    #[tracable_parser]
    pub fn parse(input: Span) -> ParserResult<Header> {
        map(
            tuple((
                Self::parse_chunk_kind,
                cut(tuple((
                    Self::parse_chunk_type,
                    Self::parse_version,
                    Self::parse_length,
                    Self::parse_name_length,
                ))),
            )),
            |(chunk_kind, (chunk_type, version, length, name_length))| Header {
                chunk_kind,
                chunk_type,
                version,
                length,
                name_length,
            },
        )(input)
    }

    #[tracable_parser]
    fn parse_chunk_kind(input: Span) -> ParserResult<String> {
        map(alt((tag("DATA"), tag("FOLD"))), |s: Span| {
            String::from_utf8_lossy(s.fragment()).into_owned()
        })(input)
    }

    #[tracable_parser]
    fn parse_chunk_type(input: Span) -> ParserResult<String> {
        parse_utf8_fixed(4usize)(input)
    }

    #[tracable_parser]
    fn parse_version(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }

    #[tracable_parser]
    fn parse_length(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }

    #[tracable_parser]
    fn parse_name_length(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }
}
