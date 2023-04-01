use crate::data::ticks::{CommandTick, MessageTick};
use crate::data::{ParserResult, Span};
use nom::branch::alt;

#[derive(Debug)]
pub enum Tick {
    Command(CommandTick),
    Message(MessageTick),
}

impl Tick {
    pub fn parse(input: Span) -> ParserResult<Tick> {
        alt((CommandTick::parse_tick, MessageTick::parse_tick))(input)
    }
}
