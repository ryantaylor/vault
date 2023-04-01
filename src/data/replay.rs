use crate::data::chunks::Chunk::{DataData, DataSdsc};
use crate::data::chunks::{Chunk, DataDataChunk, DataSdscChunk};
use crate::data::ticks::Tick::Command;
use crate::data::ticks::{CommandTick, Tick};
use crate::data::{Chunky, Header};
use crate::data::{ParserResult, Span};
use nom::combinator::eof;
use nom::combinator::map;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct Replay {
    pub header: Header,
    pub chunkies: Vec<Chunky>,
    pub chunks: Vec<Chunk>,
    pub ticks: Vec<Tick>,
}

impl Replay {
    #[tracable_parser]
    pub fn from_span(input: Span) -> ParserResult<Replay> {
        let (input, header) = Header::parse_header(input)?;

        let mut parser = map(
            tuple((
                Chunky::parse,
                Chunk::parse(header.version),
                Chunky::parse,
                Chunk::parse(header.version),
                Chunk::parse(header.version),
                many_till(Tick::parse, eof),
            )),
            |(
                first_chunky,
                foldpost_chunk,
                second_chunky,
                foldinfo_chunk,
                datasdsc_chunk,
                (ticks, _),
            )| {
                Replay {
                    header: header.clone(),
                    chunkies: vec![first_chunky, second_chunky],
                    chunks: vec![foldpost_chunk, foldinfo_chunk, datasdsc_chunk],
                    ticks,
                }
            },
        );

        parser(input)
    }

    pub fn data_chunks(&self) -> Vec<&Chunk> {
        self.chunks
            .iter()
            .flat_map(|chunk| match chunk {
                Chunk::Fold(fold) => fold.chunks.iter().collect(),
                _ => vec![chunk],
            })
            .collect()
    }

    pub fn game_data(&self) -> &DataDataChunk {
        let chunks = self.data_chunks();

        let data_chunk = chunks
            .iter()
            .find(|chunk| matches!(chunk, DataData(_)))
            .unwrap();

        match data_chunk {
            DataData(data) => data,
            _ => panic!(),
        }
    }

    pub fn map_data(&self) -> &DataSdscChunk {
        let chunks = self.data_chunks();

        let map_chunk = chunks
            .iter()
            .find(|chunk| matches!(chunk, DataSdsc(_)))
            .unwrap();

        match map_chunk {
            DataSdsc(map) => map,
            _ => panic!(),
        }
    }

    pub fn ticks(&self) -> Vec<&Tick> {
        self.ticks.iter().collect()
    }

    pub fn commands(&self) -> impl Iterator<Item = &CommandTick> {
        self.ticks
            .iter()
            .filter(|tick| matches!(tick, Command(_)))
            .map(|tick| match tick {
                Command(command) => command,
                _ => panic!(),
            })
    }
}
