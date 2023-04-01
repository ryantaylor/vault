use data::ticks::Tick;
use data::Player as PlayerData;
use message::{messages_from_data, Message};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "Vault::Player"))]
pub struct Player {
    name: String,
    faction: Faction,
    team: Team,
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
    pub fn team_id(&self) -> Team {
        self.team
    }
    pub fn steam_id(&self) -> u64 {
        self.steam_id
    }
    pub fn profile_id(&self) -> u64 {
        self.profile_id
    }
    pub fn messages(&self) -> Vec<Message> {
        self.messages.clone()
    }
}

pub fn player_from_data(player_data: &PlayerData, ticks: Vec<&Tick>) -> Player {
    Player {
        name: player_data.name.clone(),
        faction: Faction::try_from(player_data.faction.as_ref()).unwrap(),
        team: Team::try_from(player_data.team).unwrap(),
        steam_id: str::parse(&player_data.steam_id).unwrap(),
        profile_id: player_data.profile_id,
        messages: messages_from_data(ticks, &player_data.name),
    }
}

// this is safe as Player does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for Player {}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "Vault::Faction"))]
pub enum Faction {
    Americans,
    British,
    Wehrmacht,
    AfrikaKorps,
}

impl Display for Faction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Faction::Americans => write!(f, "americans"),
            Faction::British => write!(f, "british_africa"),
            Faction::Wehrmacht => write!(f, "germans"),
            Faction::AfrikaKorps => write!(f, "afrika_korps"),
        }
    }
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

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "Vault::Team"))]
pub struct Team(u32);

impl Team {
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for Team {
    type Error = String;

    fn try_from(input: u32) -> Result<Team, Self::Error> {
        match input {
            0 => Ok(Team(input)),
            1 => Ok(Team(input)),
            _ => Err(format!("Invalid team ID {}!", input)),
        }
    }
}
