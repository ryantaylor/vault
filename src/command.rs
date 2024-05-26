//! Wrapper for Company of Heroes 3 player commands.

use crate::{
    command_data::{Pbgid, Sourced, SourcedIndex, SourcedPbgid, Unknown},
    command_type::CommandType,
    data::ticks,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Wrapper for one of many Company of Heroes 3 player commands parsed from a replay file. For
/// details on the specifics of a given command, see the specific enum variants.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "VaultCoh::Command"))]
pub enum Command {
    BuildGlobalUpgrade(Pbgid),
    BuildSquad(SourcedPbgid),
    CancelConstruction(Sourced),
    CancelProduction(SourcedIndex),
    SelectBattlegroup(Pbgid),
    SelectBattlegroupAbility(Pbgid),
    UseAbility(SourcedPbgid),
    UseBattlegroupAbility(Pbgid),
    Unknown(Unknown),
}

impl Command {
    pub(crate) fn from_data_command_at_tick(command: ticks::Command, tick: u32) -> Self {
        match command.data {
            ticks::CommandData::Pbgid(pbgid) => match command.action_type {
                CommandType::CMD_Upgrade => Self::BuildGlobalUpgrade(Pbgid::new(tick, pbgid)),
                CommandType::PCMD_Ability => Self::UseBattlegroupAbility(Pbgid::new(tick, pbgid)),
                CommandType::PCMD_InstantUpgrade => {
                    Self::SelectBattlegroup(Pbgid::new(tick, pbgid))
                }
                CommandType::PCMD_TentativeUpgrade => {
                    Self::SelectBattlegroupAbility(Pbgid::new(tick, pbgid))
                }
                _ => panic!(
                    "a pbgid command isn't being handled here! command type {:?}",
                    command.action_type
                ),
            },
            ticks::CommandData::SourcedPbgid(pbgid, source_identifier) => match command.action_type
            {
                CommandType::CMD_Ability => {
                    Self::UseAbility(SourcedPbgid::new(tick, pbgid, source_identifier))
                }
                CommandType::CMD_BuildSquad => {
                    Self::BuildSquad(SourcedPbgid::new(tick, pbgid, source_identifier))
                }
                _ => panic!(
                    "a sourced pbgid command isn't being handled here! command type {:?}",
                    command.action_type
                ),
            },
            ticks::CommandData::Sourced(source_identifier) => match command.action_type {
                CommandType::CMD_CancelConstruction => Self::CancelConstruction(Sourced::new(tick, source_identifier)),
                _ => panic!(
                    "a sourced command isn't being handled here! command type {:?}",
                    command.action_type
                ),
            },
            ticks::CommandData::SourcedIndex(source_identifier, queue_index) => match command.action_type {
                CommandType::CMD_CancelProduction => Self::CancelProduction(SourcedIndex::new(tick, source_identifier, queue_index)),
                _ => panic!(
                    "a sourced command isn't being handled here! command type {:?}",
                    command.action_type
                ),
            }
            ticks::CommandData::Unknown => Self::Unknown(Unknown::new(tick, command.action_type)),
        }
    }
}

// this is safe as Command does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for Command {}
