//! Wrapper for Company of Heroes 3 player commands.

use crate::commands::build_squad::from_data as build_squad_from_data;
use crate::commands::unknown::from_data as unknown_from_data;
use crate::commands::Command::{BuildSquadCommand, UnknownCommand};
use crate::commands::{BuildSquad, Unknown};
use crate::data::commands::CommandData;
use crate::data::commands::CommandData::{BuildSquadData, UnknownCommandData};
use crate::data::ticks::Tick;
use crate::data::ticks::Tick::Command as CommandEnum;
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
        magnus(class = "VaultCoh::Commands::BuildSquadCommand")
    )]
    BuildSquadCommand(BuildSquad),
    #[cfg_attr(
        feature = "magnus",
        magnus(class = "VaultCoh::Commands::UnknownCommand")
    )]
    UnknownCommand(Unknown),
}

impl Command {
    #[cfg(feature = "magnus")]
    pub fn extract_build_squad(&self) -> BuildSquad {
        if let BuildSquadCommand(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }

    #[cfg(feature = "magnus")]
    pub fn extract_unknown(&self) -> Unknown {
        if let UnknownCommand(command) = self {
            command.clone()
        } else {
            panic!()
        }
    }
}

pub(crate) fn command_from_data(data: &CommandData, tick: i32) -> (u8, Command) {
    match data {
        BuildSquadData(build_squad) => (
            build_squad.player_id,
            BuildSquadCommand(build_squad_from_data(build_squad, tick)),
        ),
        UnknownCommandData(unknown) => (
            unknown.player_id,
            UnknownCommand(unknown_from_data(unknown, tick)),
        ),
    }
}

pub(crate) fn commands_from_data(data: &[&Tick], player_id: u32) -> Vec<Command> {
    let mut tick_count = 0;

    data.iter()
        .flat_map(|tick| {
            tick_count += 1;

            match tick {
                CommandEnum(command_tick) => command_tick
                    .bundles
                    .iter()
                    .map(|bundle| {
                        let (command_player_id, command) =
                            command_from_data(&bundle.command, tick_count);
                        if player_id == command_player_id as u32 {
                            Some(command)
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => vec![None],
            }
        })
        .filter(|entry| matches!(entry, Some(_)))
        .map(|entry| match entry {
            Some(command) => command,
            None => panic!(),
        })
        .collect()
}

// this is safe as Command does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for Command {}
