use crate::data::parser::parse_utf16_variable;
use crate::data::{ParserResult, Span};
use nom::combinator::{cut, map};
use nom::number::complete::le_u32;
use nom::sequence::tuple;

#[derive(Debug)]
pub struct Command {
    pub action_type: u8,
    pub player_id: u8,
    pub message: String,
}

impl Message {
    pub fn parse_message(input: Span) -> ParserResult<Message> {
        cut(map(
            tuple((parse_utf16_variable(le_u32), parse_utf16_variable(le_u32))),
            |((_, name), (_, message))| Message { name, message },
        ))(input)
    }
}
