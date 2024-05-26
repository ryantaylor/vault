use crate::command_type::CommandType;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Unknown {
    tick: u32,
    action_type: CommandType,
}

impl Unknown {
    pub(crate) fn new(tick: u32, action_type: CommandType) -> Self {
        Self { tick, action_type }
    }

    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// This value identifies the type of the command (build, move, stop, etc.). Commands with
    /// similar functionality can be grouped by this value.
    pub fn action_type(&self) -> CommandType {
        self.action_type
    }
}
