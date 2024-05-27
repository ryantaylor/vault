use crate::{
    command_type::CommandType,
    data::{ParserResult, Span},
};
use nom::{
    bytes::complete::take,
    combinator::{cut, flat_map, map, rest},
    number::complete::{le_u16, le_u32, le_u8},
    sequence::tuple,
};

#[derive(Debug, Copy, Clone)]
pub enum CommandData {
    Pbgid(u32),
    SourcedPbgid(u32, u16),
    Sourced(u16),
    SourcedIndex(u16, u32),
    Unknown,
}

impl CommandData {
    pub fn parse_pbgid(input: Span) -> ParserResult<CommandData> {
        map(tuple((take(31u32), le_u32)), |(_, pbgid)| {
            CommandData::Pbgid(pbgid)
        })(input)
    }

    pub fn parse_sourced_pbgid(input: Span) -> ParserResult<CommandData> {
        map(
            tuple((take(26u32), le_u16, take(3u32), le_u32)),
            |(_, source_identifier, _, pbgid)| CommandData::SourcedPbgid(pbgid, source_identifier),
        )(input)
    }

    pub fn parse_sourced(input: Span) -> ParserResult<CommandData> {
        map(tuple((take(26u32), le_u16)), |(_, source_identifier)| {
            CommandData::Sourced(source_identifier)
        })(input)
    }

    pub fn parse_sourced_index(input: Span) -> ParserResult<CommandData> {
        map(
            tuple((take(26u32), le_u16, take(2u32), le_u32)),
            |(_, source_identifier, _, queue_index)| {
                CommandData::SourcedIndex(source_identifier, queue_index)
            },
        )(input)
    }

    pub fn parse_unknown(input: Span) -> ParserResult<CommandData> {
        map(rest, |_| CommandData::Unknown)(input)
    }

    pub fn parser_for_type(
        command_type: CommandType,
    ) -> impl FnMut(Span) -> ParserResult<CommandData> {
        match command_type {
            CommandType::CMD_Upgrade
            | CommandType::PCMD_Ability
            | CommandType::PCMD_InstantUpgrade
            | CommandType::PCMD_TentativeUpgrade => Self::parse_pbgid,
            CommandType::CMD_BuildSquad | CommandType::CMD_Ability => Self::parse_sourced_pbgid,
            CommandType::CMD_CancelConstruction => Self::parse_sourced,
            CommandType::CMD_CancelProduction => Self::parse_sourced_index,
            _ => Self::parse_unknown,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub action_type: CommandType,
    pub player_id: u8,
    pub data: CommandData,
}

impl Command {
    pub fn parse(input: Span) -> ParserResult<Command> {
        cut(map(
            tuple((take(2u32), flat_map(CommandType::parse, Self::parse_type))),
            |(_, command)| command,
        ))(input)
    }

    fn parse_type(action_type: CommandType) -> impl FnMut(Span) -> ParserResult<Command> {
        move |input: Span| {
            map(
                tuple((le_u8, CommandData::parser_for_type(action_type))),
                |(player_id, data)| Command {
                    action_type,
                    player_id,
                    data,
                },
            )(input)
        }
    }
}
