use crate::data::commands::{BuildSquad, Unknown};
use crate::data::{ParserResult, Span};
use nom::branch::alt;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub enum CommandData {
    BuildSquadData(BuildSquad),
    UnknownCommandData(Unknown),
}

impl CommandData {
    #[tracable_parser]
    pub fn parse(input: Span) -> ParserResult<CommandData> {
        alt((BuildSquad::parse_command, Unknown::parse_command))(input)
    }
}
