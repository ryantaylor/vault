use crate::data::chunks::{Chunk, Chunk::Data, Header};
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::map;

#[derive(Debug)]
pub struct TrashDataChunk {
    _header: Header,
    _data: Vec<u8>,
}

impl TrashDataChunk {
    pub fn parse(input: Span, header: Header) -> ParserResult<Chunk> {
        map(take(header.length), |data: Span| {
            Data(TrashDataChunk {
                _header: header.clone(),
                _data: data.to_vec(),
            })
        })(input)
    }
}
