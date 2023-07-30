//! Representation of an unknown command.

use crate::data::commands::Unknown as UnknownData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of an unknown/unhandled command in a Company of Heroes 3 replay. All commands
/// that have not yet had parsing implemented will appear as unknown until parsing has been added.
/// New command parsing will be introduced gradually in major version updates to the library.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "magnus",
    magnus::wrap(class = "VaultCoh::Commands::Unknown")
)]
pub struct Unknown {
    tick: u32,
    action_type: u8,
}

impl Unknown {
    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// This value identifies the type of the command (build, move, stop, etc.). Commands with
    /// similar functionality can be grouped by this value.
    pub fn action_type(&self) -> u8 {
        self.action_type
    }
}

pub fn from_data(data: &UnknownData, tick: i32) -> Unknown {
    Unknown {
        tick: tick as u32,
        action_type: data.action_type,
    }
}
