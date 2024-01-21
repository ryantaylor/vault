use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{eof, map, peek};
use nom::multi::many_till;
use nom::number::complete::le_u8;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::data::ticks::Tick;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Raw {
    pub tick: u32,
    pub action_type: u8,
    pub player_id: u8,
    pub contents: Vec<u8>
}

impl Raw {
    #[tracable_parser]
    pub fn parse_command(input: Span) -> ParserResult<Raw> {
        map(
            tuple((
                peek(many_till(le_u8, eof)),
                take(2u32),
                le_u8,
                le_u8,
            )),
            |((contents, _), _, action_type, player_id)| Raw { action_type, player_id, contents, tick: 0 },
        )(input)
    }
}

pub(crate) fn raw_from_data(data: &[&Tick], player_id: u32) -> Vec<Raw> {
    let mut tick_count = 0;

    data.iter()
        .flat_map(|tick| {
            tick_count += 1;

            match tick {
                Tick::Command(command_tick) => command_tick
                    .bundles
                    .iter()
                    .map(|bundle| {
                        if player_id == bundle.raw.player_id as u32 {
                            let mut command = bundle.raw.clone();
                            command.tick = tick_count;
                            Some(command)
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => vec![None],
            }
        })
        .filter(|entry| entry.is_some())
        .map(|entry| match entry {
            Some(raw) => raw,
            None => panic!(),
        })
        .collect()
}
