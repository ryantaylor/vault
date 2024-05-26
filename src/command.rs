//! Wrapper for Company of Heroes 3 player commands.

use crate::{
    command_data::{Pgbid, Unknown},
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
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::BuildGlobalUpgradeCommand")
    )]
    BuildGlobalUpgrade(Pgbid),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::BuildSquadCommand")
    )]
    BuildSquad(Pgbid),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::SelectBattlegroupCommand")
    )]
    SelectBattlegroup(Pgbid),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::SelectBattlegroupAbilityCommand")
    )]
    SelectBattlegroupAbility(Pgbid),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::UseBattlegroupAbilityCommand")
    )]
    UseBattlegroupAbility(Pgbid),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::UnknownCommand")
    )]
    Unknown(Unknown),
}

impl Command {
    pub(crate) fn from_data_command_at_tick(command: ticks::Command, tick: u32) -> Self {
        match command.data {
            ticks::CommandData::Pgbid(pgbid) => match command.action_type {
                CommandType::CMD_BuildSquad => Self::BuildSquad(Pgbid::new(tick, pgbid)),
                CommandType::CMD_Upgrade => Self::BuildGlobalUpgrade(Pgbid::new(tick, pgbid)),
                CommandType::PCMD_Ability => Self::UseBattlegroupAbility(Pgbid::new(tick, pgbid)),
                CommandType::PCMD_InstantUpgrade => {
                    Self::SelectBattlegroup(Pgbid::new(tick, pgbid))
                }
                CommandType::PCMD_TentativeUpgrade => {
                    Self::SelectBattlegroupAbility(Pgbid::new(tick, pgbid))
                }
                _ => panic!(
                    "a pgbid command isn't being handled here! command type {:?}",
                    command.action_type
                ),
            },
            ticks::CommandData::Unknown => Self::Unknown(Unknown::new(tick, command.action_type)),
        }
    }
}

// this is safe as Command does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for Command {}
