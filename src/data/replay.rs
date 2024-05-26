use crate::command::Command;
use crate::data::chunks::Chunk::{DataAuto, DataData, DataSdsc};
use crate::data::chunks::{Chunk, DataAutoChunk, DataDataChunk, DataSdscChunk};
use crate::data::ticks::{CommandTick, Tick};
use crate::data::{Chunky, Header};
use crate::data::{ParserResult, Span};
use crate::Message;
use nom::combinator::eof;
use nom::combinator::map;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;
use std::collections::HashMap;

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

    pub fn automatch_data(&self) -> Option<&DataAutoChunk> {
        match self
            .data_chunks()
            .iter()
            .find(|chunk| matches!(chunk, DataAuto(_)))
        {
            Some(DataAuto(chunk)) => Some(chunk),
            None => None,
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

    pub fn command_ticks(&self) -> impl Iterator<Item = &CommandTick> {
        self.ticks.iter().filter_map(|tick| match tick {
            Tick::Command(command) => Some(command),
            _ => None,
        })
    }

    pub fn commands(&self) -> HashMap<u32, Vec<Command>> {
        self.command_ticks()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (idx, tick)| {
                for bundle in tick.bundles.iter() {
                    let commands = acc.entry(bundle.command.player_id as u32).or_default();
                    commands.push(Command::from_data_command_at_tick(
                        bundle.command,
                        idx as u32 + 1,
                    ));
                }
                acc
            })
    }

    pub fn messages(&self) -> HashMap<String, Vec<Message>> {
        self.ticks
            .iter()
            .enumerate()
            .filter_map(|(idx, tick)| match tick {
                Tick::Message(message) => Some((idx + 1, message.messages.clone())),
                _ => None,
            })
            .fold(HashMap::new(), |mut acc, (tick, messages)| {
                for message in messages.iter() {
                    let msgs = acc.entry(message.name.clone()).or_default();
                    msgs.push(Message::new(tick as u32, message.message.clone()));
                }
                acc
            })
    }
}
