//! Representation of a unit production cancellation command.

use crate::data::commands::CancelProduction as CancelProductionData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of a unit production cancellation command in a Company of Heroes 3 replay.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "magnus",
    magnus::wrap(class = "VaultCoh::Commands::CancelProduction")
)]
pub struct CancelProduction {
    tick: u32,
    source_identifier: u16,
    queue_index: u32,
}

impl CancelProduction {
    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// This value corresponds to the internal identifier given by the game engine to the structure
    /// responsible for producing the unit that has been cancelled. If you know the identifier
    /// for a given structure, you can tie the cancellation command to that structure. Since build
    /// commands also have source identifiers, this value can be used alongside queue index to
    /// determine which specific build command is being cancelled.
    pub fn source_identifier(&self) -> u16 {
        self.source_identifier
    }
    /// The index of the position in the source structure's build queue that this cancellation command
    /// corresponds to. Every time a build command is issued, the command is added to the source
    /// structure's build queue and given an index. These indexes start at 1 and increase by 1 every
    /// time a new build command is issued. This value can be used alongside source identifier to
    /// determine which specific build command is being cancelled.
    pub fn queue_index(&self) -> u32 {
        self.queue_index
    }
}

pub fn from_data(data: &CancelProductionData, tick: i32) -> CancelProduction {
    CancelProduction {
        tick: tick as u32,
        source_identifier: data.source_identifier,
        queue_index: data.queue_index,
    }
}

// this is safe as CancelProduction does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for CancelProduction {}
