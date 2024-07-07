#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A command format with both a source identifier and a queue index.

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourcedIndex {
    tick: u32,
    index: u32,
    source_identifier: u16,
    queue_index: u32,
}

impl SourcedIndex {
    pub(crate) fn new(tick: u32, index: u32, source_identifier: u16, queue_index: u32) -> Self {
        Self {
            tick,
            index,
            source_identifier,
            queue_index,
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
    /// This value corresponds to the internal identifier given by the game engine to the entity
    /// that is the source of the command. If you know the identifier for a given entity, you can
    /// use this value to link this command to that entity.
    pub fn source_identifier(&self) -> u16 {
        self.source_identifier
    }
    /// The index of the position in the source entity's build queue that this command corresponds
    /// to. Usually used with build and cancellation commands, every time a build command is issued,
    /// the command is added to the source structure's build queue and given an index. These indexes
    /// start at 1 and increase by 1 every time a new build command is issued. This value can be used
    /// alongside source identifier to determine which specific build command is being cancelled.
    pub fn queue_index(&self) -> u32 {
        self.queue_index
    }
}
