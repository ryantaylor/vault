use std::string::String;

pub struct Player {
    name: String,
    steam_id: u64,
    team: u32,
    faction: String,
    commanders: Vec<u32>,
    bulletin_ids: Vec<u32>,
    bulletin_names: Vec<String>,
    victory_strike: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: String::new(),
            steam_id: 0,
            team: 0,
            faction: String::new(),
            commanders: Vec::with_capacity(3),
            bulletin_ids: Vec::with_capacity(3),
            bulletin_names: Vec::with_capacity(3),
            victory_strike: 0,
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

    pub fn add_commander(&mut self, id: u32) {
        trace!("Player::add_commander");
        self.commanders.push(id);
    }

    pub fn add_bulletin(&mut self, id: u32) {
        trace!("Player::add_bulletin");
        self.bulletin_ids.push(id);
    }

    pub fn add_bulletin_name<S>(&mut self, name: S) where S: Into<String> {
        trace!("Player::add_bulletin_name");
        self.bulletin_names.push(name.into());
    }

    pub fn update_victory_strike(&mut self, id: u32) {
        trace!("Player::update_victory_strike");
        self.victory_strike = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn display(&self) {
        println!("name: {}", self.name);
        println!("steam_id: {}", self.steam_id);
        println!("team: {}", self.team);
        println!("faction: {}", self.faction);
    }
}