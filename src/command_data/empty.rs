#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An empty command format with no additional context.

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Empty {
    tick: u32,
}

impl Empty {
    pub(crate) fn new(tick: u32) -> Self {
        Self { tick }
    }

    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
}
