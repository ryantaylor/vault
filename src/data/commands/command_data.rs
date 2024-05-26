use crate::data::commands::{
    BuildGlobalUpgrade, BuildSquad, CancelConstruction, CancelProduction, SelectBattlegroup,
    SelectBattlegroupAbility, Unknown, UseAbility, UseBattlegroupAbility,
};
use crate::data::{ParserResult, Span};
use nom::branch::alt;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub enum CommandData {
    BuildGlobalUpgrade(BuildGlobalUpgrade),
    BuildSquad(BuildSquad),
    CancelConstruction(CancelConstruction),
    CancelProduction(CancelProduction),
    SelectBattlegroup(SelectBattlegroup),
    SelectBattlegroupAbility(SelectBattlegroupAbility),
    UseAbility(UseAbility),
    UseBattlegroupAbility(UseBattlegroupAbility),
    Unknown(Unknown),
}

impl CommandData {
    #[tracable_parser]
    pub fn parse(input: Span) -> ParserResult<CommandData> {
        alt((
            BuildSquad::parse_command,
            BuildGlobalUpgrade::parse_command,
            UseAbility::parse_command,
            UseBattlegroupAbility::parse_command,
            CancelConstruction::parse_command,
            CancelProduction::parse_command,
            SelectBattlegroupAbility::parse_command,
            SelectBattlegroup::parse_command,
            Unknown::parse_command,
        ))(input)
    }
}
