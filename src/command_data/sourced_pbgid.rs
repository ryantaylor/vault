#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A command format with both an entity pbgid and a source identifier.

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourcedPbgid {
    tick: u32,
    index: u32,
    pbgid: u32,
    source_identifier: u16,
}

impl SourcedPbgid {
    pub(crate) fn new(tick: u32, index: u32, pbgid: u32, source_identifier: u16) -> Self {
        Self {
            tick,
            index,
            pbgid,
            source_identifier,
        }
    }

    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// This value is the index of the command relative to the player who issued the command.
    /// Indexes start at 1 and increment on every player-issued command, which means you should be
    /// able to look at the maximum index value of the commands associated with a player to
    /// determine how many commands that player issued in a given game.
    pub fn index(&self) -> u32 {
        self.index
    }
    /// Internal ID that uniquely identifies entity associated with the command. This value can be
    /// matched to CoH3 attribute files in order to determine the entity in question. Note that,
    /// while rare, it is possible that this value may change between patches for the same entity.
    pub fn pbgid(&self) -> u32 {
        self.pbgid
    }
    /// This value corresponds to the internal identifier given by the game engine to the entity
    /// that is the source of the command. If you know the identifier for a given entity, you can
    /// use this value to link this command to that entity.
    pub fn source_identifier(&self) -> u16 {
        self.source_identifier
    }
}
