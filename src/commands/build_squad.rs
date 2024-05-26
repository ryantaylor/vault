//! Representation of a unit build command.

use crate::data::commands::BuildSquad as BuildSquadData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of a unit build/construction command in a Company of Heroes 3 replay. Note that
/// this command logically represents the enqueuing of a build command within the game engine and
/// does not necessarily guarantee that the unit in question was actually built. If a command to
/// cancel construction is issued before the unit is fielded, the unit cost will be refunded.
/// Therefore, to calculate build orders, one must match source identifiers and queue indexes from
/// build and cancellation commands in order to filter out cancelled units.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`. To quickly access all of a player's build
/// commands, see `Player::build_commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "magnus",
    magnus::wrap(class = "VaultCoh::Commands::BuildSquad")
)]
pub struct BuildSquad {
    tick: u32,
    pbgid: u32,
    source_identifier: u16,
}

impl BuildSquad {
    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// This value corresponds to the internal identifier given by the game engine to the building
    /// this build command was issued at. If the unit being built is later cancelled, the
    /// cancellation command's source identifier will match this value.
    pub fn source_identifier(&self) -> u16 {
        self.source_identifier
    }
    /// Internal ID that uniquely identifies the unit being built. This value can be matched to
    /// CoH3 attribute files in order to determine the unit being built. Note that, while rare, it
    /// is possible that this value may change between patches for the same unit.
    pub fn pbgid(&self) -> u32 {
        self.pbgid
    }
}

pub fn from_data(data: &BuildSquadData, tick: i32) -> BuildSquad {
    BuildSquad {
        tick: tick as u32,
        pbgid: data.pgbid,
        source_identifier: data.source_identifier,
    }
}

// this is safe as BuildSquad does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for BuildSquad {}
