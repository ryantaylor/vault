use crate::data::chunks::{Chunk, Chunk::Data, Header};
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::map;

#[derive(Debug)]
pub struct TrashDataChunk {
    pub header: Header,
    pub data: Vec<u8>,
}

impl TrashDataChunk {
    pub fn parse(input: Span, header: Header) -> ParserResult<Chunk> {
        map(take(header.length), |data: Span| {
            Data(TrashDataChunk {
                header: header.clone(),
                data: data.to_vec(),
            })
        })(input)
    }
}
