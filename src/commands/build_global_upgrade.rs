//! Representation of a global upgrade build command.

use crate::data::commands::BuildGlobalUpgrade as BuildGlobalUpgradeData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of a global upgrade build command in a Company of Heroes 3 replay. Note that
/// this command logically represents the enqueuing of an upgrade command within the game engine and
/// does not necessarily guarantee that the upgrade in question was actually built. If a command to
/// cancel construction is issued before the upgrade is completed, the upgrade cost will be
/// refunded. Therefore, to calculate build orders, one must be aware of all upgrade build times for
/// the corresponding patch version and track upgrade cancellation as well.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`. To quickly access all of a player's build
/// commands, see `Player::build_commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "magnus",
    magnus::wrap(class = "VaultCoh::Commands::BuildGlobalUpgrade")
)]
pub struct BuildGlobalUpgrade {
    tick: u32,
    pbgid: u32,
}

impl BuildGlobalUpgrade {
    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// Internal ID that uniquely identifies the upgrade being built. This value can be matched to
    /// CoH3 attribute files in order to determine the upgrade being built. Note that, while rare,
    /// it is possible that this value may change between patches for the same upgrade.
    pub fn pbgid(&self) -> u32 {
        self.pbgid
    }
}

pub fn from_data(data: &BuildGlobalUpgradeData, tick: i32) -> BuildGlobalUpgrade {
    BuildGlobalUpgrade {
        tick: tick as u32,
        pbgid: data.pgbid,
    }
}

// this is safe as BuildGlobalUpgrade does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for BuildGlobalUpgrade {}
