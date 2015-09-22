//! A module containing a representation of a player entity in CoH2 replays.

use std::string::String;

use item::Item;

/// This type represents a Company of Heroes 2 player entity as it appears in a CoH2 replay file.

#[derive(Debug, RustcEncodable)]
pub struct Player {
    name: String,
    steam_id: u64,
    team: u32,
    faction: String,
    items: Vec<Item>
}

impl Player {

    /// Constructs a new Player with empty initial data.

    pub fn new() -> Player {
        Player {
            name: String::new(),
            steam_id: 0,
            team: 0,
            faction: String::new(),
            items: Vec::with_capacity(12) // cmdr x3, intel x3, skin x3, decal, strike, faceplate
        }
    }

    /// Updates the Player's name.

    pub fn update_name<S>(&mut self, name: S) where S: Into<String> {
        trace!("Player::update_name");
        self.name = name.into();
    }

    /// Updates the Player's Steam ID.

    pub fn update_steam_id(&mut self, id: u64) {
        trace!("Player::update_steam_id");
        self.steam_id = id;
    }

    /// Updates the Player's team.

    pub fn update_team(&mut self, id: u32) {
        trace!("Player::update_team");
        self.team = id;
    }

    /// Updates the Player's faction.

    pub fn update_faction<S>(&mut self, faction: S) where S: Into<String> {
        trace!("Player::update_faction");
        self.faction = faction.into();
    }

    /// Adds an Item to the Player's list.

    pub fn add_item(&mut self, item: Item) {
        trace!("Player::add_item");
        self.items.push(item);
    }

    /// Returns the Player's name;

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Writes the contents of the Player to stdout.

    #[allow(dead_code)]
    pub fn display(&self) {
        println!("name: {}", self.name);
        println!("steam_id: {}", self.steam_id);
        println!("team: {}", self.team);
        println!("faction: {}", self.faction);

        for item in self.items.iter() {
            println!("{:?}: {}", item.item_type(), item.id());
        }
    }
}