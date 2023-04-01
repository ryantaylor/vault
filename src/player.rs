use data::ticks::Tick;
use data::Player as PlayerData;
use message::{messages_from_data, Message};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "Vault::Player"))]
pub struct Player {
    name: String,
    faction: Faction,
    steam_id: u64,
    profile_id: u64,
    messages: Vec<Message>,
}

impl Player {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn faction(&self) -> Faction {
        self.faction
    }
    pub fn steam_id(&self) -> u64 {
        self.steam_id
    }
    pub fn profile_id(&self) -> u64 {
        self.profile_id
    }
    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }
}

pub fn player_from_data(player_data: &PlayerData, ticks: Vec<&Tick>) -> Player {
    Player {
        name: player_data.name.clone(),
        faction: Faction::try_from(player_data.faction.as_ref()).unwrap(),
        steam_id: str::parse(&player_data.steam_id).unwrap(),
        profile_id: player_data.profile_id,
        messages: messages_from_data(ticks, &player_data.name),
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Faction {
    Americans,
    British,
    Wehrmacht,
    AfrikaKorps,
}

impl TryFrom<&str> for Faction {
    type Error = String;

    fn try_from(input: &str) -> Result<Faction, Self::Error> {
        match input {
            "americans" => Ok(Faction::Americans),
            "british_africa" => Ok(Faction::British),
            "germans" => Ok(Faction::Wehrmacht),
            "afrika_korps" => Ok(Faction::AfrikaKorps),
            _ => Err(format!("Invalid faction type {}!", input)),
        }
    }
}
