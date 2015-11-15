//! A module containing a representation of a player entity in CoH2 replays.

use std::string::String;

use command::Command;
use item::Item;

/// This type represents a Company of Heroes 2 player entity as it appears in a CoH2 replay file.

#[derive(Debug, RustcEncodable)]
pub struct Player {
    pub id: u8,
    pub name: String,
    pub steam_id: u64,
    pub team: u32,
    pub faction: String,
    pub items: Vec<Item>,
    pub commands: Vec<Command>,
    pub cpm: f64,
}

impl Player {

    /// Constructs a new `Player` with empty initial data.

    pub fn new(id: u8) -> Player {
        Player {
            id: id,
            name: String::new(),
            steam_id: 0,
            team: 0,
            faction: String::new(),
            items: Vec::with_capacity(12), // cmdr x3, intel x3, skin x3, decal, strike, faceplate
            commands: Vec::new(),
            cpm: 0.0,
        }
    }

    /// Writes the contents of the `Player` to `stdout`.

    pub fn display(&self) {
        println!("id: {}", self.id);
        println!("name: {}", self.name);
        println!("steam_id: {}", self.steam_id);
        println!("team: {}", self.team);
        println!("faction: {}", self.faction);
        println!("cpm: {}", self.cpm);

        for item in &self.items {
            println!("{:?}: {}", item.item_type, item.id);
        }
    }
}