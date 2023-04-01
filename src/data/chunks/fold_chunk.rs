use data::chunks::{Chunk, Chunk::Fold, Header};
use data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{cut, eof, map, map_parser};
use nom::multi::many0;
use nom::sequence::terminated;

#[derive(Debug)]
pub struct FoldChunk {
    pub header: Header,
    pub chunks: Vec<Chunk>,
}

impl FoldChunk {
    pub fn parse(input: Span, header: Header, version: u16) -> ParserResult<Chunk> {
        cut(map_parser(
            take(header.length),
            terminated(
                map(
                    map_parser(take(header.length), many0(Chunk::parse(version))),
                    move |chunks| {
                        Fold(FoldChunk {
                            header: header.clone(),
                            chunks,
                        })
                    },
                ),
                eof,
            ),
        ))(input)
    }
}
