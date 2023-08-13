//! Representation of a battlegroup selection command.

use crate::data::commands::SelectBattlegroup as SelectBattlegroupData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of a battlegroup selection command in a Company of Heroes 3 replay.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`. To quickly access a player's selected battlegroup
/// without having to extract it from the player's commands, see `Player::battlegroup`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "magnus",
    magnus::wrap(class = "VaultCoh::Commands::SelectBattlegroup")
)]
pub struct SelectBattlegroup {
    tick: u32,
    pbgid: u32,
}

impl SelectBattlegroup {
    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// Internal ID that uniquely identifies the battlegroup selected. This value can be matched to
    /// CoH3 attribute files in order to determine the unit being built. Note that, while rare, it
    /// is possible that this value may change between patches for the same battlegroup.
    pub fn pbgid(&self) -> u32 {
        self.pbgid
    }
}

pub fn from_data(data: &SelectBattlegroupData, tick: i32) -> SelectBattlegroup {
    SelectBattlegroup {
        tick: tick as u32,
        pbgid: data.pgbid,
    }
}

// this is safe as SelectBattlegroup does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for SelectBattlegroup {}
