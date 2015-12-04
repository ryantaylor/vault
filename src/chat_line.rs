//! A module containing a representation of chat lines in CoH2 replays.

use std::string::String;

/// This type represents a single Company of Heroes 2 chat message.

#[derive(Debug, RustcEncodable)]
pub struct ChatLine {
    pub tick: u32,
    pub name: String,
    pub message: String,
}

impl ChatLine {

    /// Constructs a new `ChatLine` initialized with the data given.

    pub fn with_data<S>(tick: u32, name: S, message: S) -> ChatLine where S: Into<String> {
        ChatLine {
            tick: tick,
            name: name.into(),
            message: message.into(),
        }
    }
}