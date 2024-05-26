//! Representation of a general ability usage command.

use crate::data::commands::UseAbility as UseAbilityData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of a general ability usage command in a Company of Heroes 3 replay.
///
/// Commands are collected during tick parsing and then associated with the `Player` instance that
/// sent them. To access, see `Player::commands`.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "magnus",
    magnus::wrap(class = "VaultCoh::Commands::UseAbility")
)]
pub struct UseAbility {
    tick: u32,
    source_identifier: u16,
    pbgid: u32,
}

impl UseAbility {
    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// This value corresponds to the internal identifier given by the game engine to the entity
    /// that was ordered to perform the ability. If you know the identifier for a given entity,
    /// you can tie the ability usage command to that entity.
    pub fn source_identifier(&self) -> u16 {
        self.source_identifier
    }
    /// Internal ID that uniquely identifies the ability used. This value can be matched to CoH3
    /// attribute files in order to determine the ability being used. Note that, while rare, it is
    /// possible that this value may change between patches for the same ability.
    pub fn pbgid(&self) -> u32 {
        self.pbgid
    }
}

pub fn from_data(data: &UseAbilityData, tick: i32) -> UseAbility {
    UseAbility {
        tick: tick as u32,
        source_identifier: data.source_identifier,
        pbgid: data.pgbid,
    }
}

// this is safe as UseAbility does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for UseAbility {}
