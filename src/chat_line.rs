// //! A module containing a representation of chat lines in CoH2 replays.
//
// use std::string::String;
//
// /// This type represents a single Company of Heroes 2 chat message.
//
// #[derive(Debug, RustcEncodable)]
// pub struct ChatLine {
//     /// Tick count where the chat message occurred
//     pub tick: u32,
//     /// Name of the player who wrote the message
//     pub name: String,
//     /// Text of the message
//     pub message: String,
// }
//
// impl ChatLine {
//
//     /// Constructs a new `ChatLine` initialized with the data given.
//
//     pub fn with_data<S>(tick: u32, name: S, message: S) -> ChatLine where S: Into<String> {
//         ChatLine {
//             tick: tick,
//             name: name.into(),
//             message: message.into(),
//         }
//     }
// }