// //! A module containing a representation of a player entity in CoH2 replays.
//
// use std::string::String;
//
// use item::Item;
//
// /// This type represents a Company of Heroes 2 player entity as it appears in a CoH2 replay file.
//
// #[derive(Debug, RustcEncodable)]
// pub struct Player {
//     /// Internal ID of player
//     pub id: u8,
//     /// Unicode name of player
//     pub name: String,
//     /// 64-bit Steam ID
//     pub steam_id: u64,
//     /// Steam ID as string because JS has trouble with true 64-bit ints
//     pub steam_id_str: String,
//     /// Team the player belongs to
//     pub team: u32,
//     /// String representation of the player's faction
//     pub faction: String,
//     /// If it can be found, the `server_id` of the player's commander
//     pub commander: u32,
//     /// A collection of the player's equipped items
//     pub items: Vec<Item>,
//     /// If it can be calculated, the player's commands per minute
//     pub cpm: f64,
// }
//
// impl Player {
//
//     /// Constructs a new `Player` with empty initial data.
//
//     pub fn new() -> Player {
//         Player {
//             id: 0xF,
//             name: String::new(),
//             steam_id: 0,
//             steam_id_str: "0".to_owned(),
//             team: 0,
//             faction: String::new(),
//             commander: 0,
//             items: Vec::with_capacity(12), // cmdr x3, intel x3, skin x3, decal, strike, faceplate
//             cpm: 0.0,
//         }
//     }
// }