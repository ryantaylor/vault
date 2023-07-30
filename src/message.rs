//! Representation of parsed message information.

use crate::data::ticks::Tick;
use crate::data::ticks::Tick::Message as MessageEnum;
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

pub fn messages_from_data(data: &Vec<&Tick>, player_name: &str) -> Vec<Message> {
    let mut tick_count = 0;

    data.iter()
        .flat_map(|tick| {
            tick_count += 1;

            match tick {
                MessageEnum(message_tick) => message_tick
                    .messages
                    .iter()
                    .map(|message| {
                        if message.name == player_name {
                            Some(Message {
                                tick: tick_count,
                                message: message.message.clone(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => vec![None],
            }
        })
        .filter(|entry| matches!(entry, Some(_)))
        .map(|entry| match entry {
            Some(message) => message,
            None => panic!(),
        })
        .collect()
}

// this is safe as Message does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for Message {}
