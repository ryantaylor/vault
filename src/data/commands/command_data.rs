use crate::data::commands::{BuildSquad, SelectBattlegroup, Unknown};
use crate::data::{ParserResult, Span};
use nom::branch::alt;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub enum CommandData {
    BuildSquad(BuildSquad),
    SelectBattlegroup(SelectBattlegroup),
    Unknown(Unknown),
}

impl CommandData {
    #[tracable_parser]
    pub fn parse(input: Span) -> ParserResult<CommandData> {
        alt((
            BuildSquad::parse_command,
            SelectBattlegroup::parse_command,
            Unknown::parse_command,
        ))(input)
    }
}
