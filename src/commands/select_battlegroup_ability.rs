//! Representation of a battlegroup ability selection command.

use crate::data::commands::SelectBattlegroupAbility as SelectBattlegroupAbilityData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of a battlegroup ability selection command in a Company of Heroes 3 replay.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`. To access all of a player's battlegroup-related
/// commands, see `Player::battlegroup_commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "magnus",
    magnus::wrap(class = "VaultCoh::Commands::SelectBattlegroupAbility")
)]
pub struct SelectBattlegroupAbility {
    tick: u32,
    pbgid: u32,
}

impl SelectBattlegroupAbility {
    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// Internal ID that uniquely identifies the battlegroup ability selected. This value can be
    /// matched to CoH3 attribute files in order to determine the battlegroup ability being
    /// selected. Note that, while rare, it is possible that this value may change between patches
    /// for the same battlegroup ability.
    pub fn pbgid(&self) -> u32 {
        self.pbgid
    }
}

pub fn from_data(data: &SelectBattlegroupAbilityData, tick: i32) -> SelectBattlegroupAbility {
    SelectBattlegroupAbility {
        tick: tick as u32,
        pbgid: data.pgbid,
    }
}

// this is safe as SelectBattlegroupAbility does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for SelectBattlegroupAbility {}
