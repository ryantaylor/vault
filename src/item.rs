// //! A module containing a representation of items in CoH2 that can be equipped, as represented in
// //! CoH2 replay files.
//
// /// This type contains the types of all potentially equipped items that can be parsed out of CoH2
// /// replays.
//
// #[derive(Debug, Copy, Clone, PartialEq, RustcEncodable)]
// pub enum ItemType {
//     Commander,
//     Bulletin,
//     Skin,
//     VictoryStrike,
//     Decal,
//     FacePlate
// }
//
// /// This type contains a parsed representation of an item that can be equipped in a CoH2 replay.
//
// #[derive(Debug, RustcEncodable)]
// pub struct Item {
//     /// Used in a `PCMD_SetCommander` command to refer to the commander selected by a player
//     pub selection_id: u32,
//     /// Corresponds to a `server_id` in Relic's attribute files
//     pub server_id: u32,
//     /// Internally used to organize different item types
//     pub item_type: ItemType,
// }
//
// impl Item {
//
//     /// Constructs a new `Item` with an empty ID and the given `ItemType`.
//
//     pub fn new(item_type: ItemType) -> Item {
//         Item {
//             selection_id: 0,
//             server_id: 0,
//             item_type: item_type,
//         }
//     }
// }