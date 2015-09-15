use std::string::String;
use item::Item;

pub struct Player {
    name: String,
    steam_id: u64,
    team: u32,
    faction: String,
    items: Vec<Item>
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: String::new(),
            steam_id: 0,
            team: 0,
            faction: String::new(),
            items: Vec::with_capacity(12) // cmdr x3, intel x3, skin x3, decal, strike, faceplate
        }
    }

    pub fn update_name<S>(&mut self, name: S) where S: Into<String> {
        trace!("Player::update_name");
        self.name = name.into();
    }

    pub fn update_steam_id(&mut self, id: u64) {
        trace!("Player::update_steam_id");
        self.steam_id = id;
    }

    pub fn update_team(&mut self, id: u32) {
        trace!("Player::update_team");
        self.team = id;
    }

    pub fn update_faction<S>(&mut self, faction: S) where S: Into<String> {
        trace!("Player::update_faction");
        self.faction = faction.into();
    }

    pub fn add_item(&mut self, item: Item) {
        trace!("Player::add_item");
        self.items.push(item);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

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