//! A module containing a representation of a player entity in CoH2 replays.

use std::string::String;

use item::Item;

/// This type represents a Company of Heroes 2 player entity as it appears in a CoH2 replay file.

#[derive(Debug, RustcEncodable)]
pub struct Player {
    pub name: String,
    pub steam_id: u64,
    pub team: u32,
    pub faction: String,
    pub items: Vec<Item>,
}

impl Player {

    /// Constructs a new Player with empty initial data.

    pub fn new() -> Player {
        Player {
            name: String::new(),
            steam_id: 0,
            team: 0,
            faction: String::new(),
            items: Vec::with_capacity(12), // cmdr x3, intel x3, skin x3, decal, strike, faceplate
        }
    }

    /// Writes the contents of the Player to stdout.

    #[allow(dead_code)]
    pub fn display(&self) {
        println!("name: {}", self.name);
        println!("steam_id: {}", self.steam_id);
        println!("team: {}", self.team);
        println!("faction: {}", self.faction);

        for item in &self.items {
            println!("{:?}: {}", item.item_type, item.id);
        }
    }
}