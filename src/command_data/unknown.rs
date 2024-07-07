use crate::command_type::CommandType;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A catch-all command format to cover commands that aren't currently being parsed.

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Unknown {
    tick: u32,
    index: u32,
    action_type: CommandType,
}

impl Unknown {
    pub(crate) fn new(tick: u32, index: u32, action_type: CommandType) -> Self {
        Self {
            tick,
            index,
            action_type,
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
    /// determine how many commands that player issued in a given game. System commands that were
    /// not triggered by player action will have an index of 0.
    pub fn index(&self) -> u32 {
        self.index
    }
    /// This value identifies the type of the command (build, move, stop, etc.). Commands with
    /// similar functionality can be grouped by this value.
    pub fn action_type(&self) -> CommandType {
        self.action_type
    }
}
