//! Representation of a construction cancellation command.

use crate::data::commands::CancelConstruction as CancelConstructionData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of a construction cancellation command in a Company of Heroes 3 replay.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "magnus",
    magnus::wrap(class = "VaultCoh::Commands::CancelConstruction")
)]
pub struct CancelConstruction {
    tick: u32,
    source_identifier: u16,
}

impl CancelConstruction {
    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// This value corresponds to the internal identifier given by the game engine to the entity
    /// responsible for building the structure that has been cancelled. If you know the identifier
    /// for a given entity, you can tie the cancellation command to that entity.
    pub fn source_identifier(&self) -> u16 {
        self.source_identifier
    }
}

pub fn from_data(data: &CancelConstructionData, tick: i32) -> CancelConstruction {
    CancelConstruction {
        tick: tick as u32,
        source_identifier: data.source_identifier,
    }
}

// this is safe as CancelConstruction does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for CancelConstruction {}
