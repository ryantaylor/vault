use crate::data::chunks::{
    DataAutoChunk, DataDataChunk, DataSdscChunk, FoldChunk, Header, TrashDataChunk,
};
use crate::data::{ParserResult, Span};

#[derive(Debug)]
pub enum Chunk {
    Fold(FoldChunk),
    Data(TrashDataChunk),
    DataAuto(DataAutoChunk),
    DataData(DataDataChunk),
    DataSdsc(DataSdscChunk),
}

impl Chunk {
    pub fn parse(version: u16) -> impl FnMut(Span) -> ParserResult<Chunk> {
        move |input: Span| {
            let (input, header) = Header::parse(input)?;

            return match &header.chunk_kind as &str {
                "DATA" => match &header.chunk_type as &str {
                    "AUTO" => DataAutoChunk::parse(input, header),
                    "DATA" => DataDataChunk::parse(input, header, version),
                    "SDSC" => DataSdscChunk::parse(input, header),
                    _ => TrashDataChunk::parse(input, header),
                },
                "FOLD" => FoldChunk::parse(input, header, version),
                _ => panic!(),
            };
        }
    }
}
