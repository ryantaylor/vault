use data::Replay as ReplayData;
use map::{map_from_data, Map};
use player::{player_from_data, Player};

#[derive(Debug)]
pub struct Replay {
    version: u16,
    timestamp: String,
    matchhistory_id: u64,
    map: Map,
    players: Vec<Player>,
    length: usize,
}

impl Replay {
    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn matchhistory_id(&self) -> u64 {
        self.matchhistory_id
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn map_filename(&self) -> &str {
        self.map.filename()
    }

    pub fn map_localized_name_id(&self) -> &str {
        self.map.localized_name_id()
    }

    pub fn map_localized_description_id(&self) -> &str {
        self.map.localized_description_id()
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

pub fn replay_from_data(data: &ReplayData) -> Replay {
    Replay {
        version: data.header.version,
        timestamp: data.header.timestamp.clone(),
        matchhistory_id: data.game_data().matchhistory_id,
        map: map_from_data(data.map_data()),
        length: data.commands().count(),
        players: data
            .game_data()
            .players
            .iter()
            .map(|player| player_from_data(player, data.ticks()))
            .collect(),
    }
}
