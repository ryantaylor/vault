//! Representation of parsed message information.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of a user-sent chat message in a Company of Heroes 3 replay. Messages are
/// collected during command parsing and then associated with the `Player` instance that sent them.
/// To access, see `Player::messages`.

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "VaultCoh::Message"))]
pub struct Message {
    tick: u32,
    message: String,
}

impl Message {
    pub(crate) fn new(tick: u32, message: String) -> Self {
        Self { tick, message }
    }

    /// This value is the tick at which the message was found while parsing the replay, which
    /// represents the time in the replay at which it was sent. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this message was sent.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// UTF-16 encoded representation of the message sent by the player.
    pub fn message(&self) -> &str {
        &self.message
    }
}

// this is safe as Message does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for Message {}
