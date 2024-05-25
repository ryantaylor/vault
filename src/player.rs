//! Representation of parsed player information.

use crate::commands::{commands_from_data, Command};
use crate::data::ticks::Tick;
use crate::data::Player as PlayerData;
use crate::message::{messages_from_data, Message};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::data::commands::{Raw, raw_from_data};

/// Game-specific player representation. Includes generally immutable information alongside data
/// specific to the replay being parsed.

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "VaultCoh::Player"))]
pub struct Player {
    name: String,
    human: bool,
    faction: Faction,
    team: Team,
    battlegroup: Option<u32>,
    steam_id: Option<u64>,
    profile_id: Option<u64>,
    messages: Vec<Message>,
    commands: Vec<Command>,
    raw_command_data: Vec<Raw>,
}

impl Player {
    /// Name of the player at the time the replay was recorded. Note that the player may have
    /// changed their name since time of recording. If attempting to uniquely identify players
    /// across replay files, look at `Player::steam_id` and `Player::profile_id` instead. The string
    /// is UTF-16 encoded.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Whether or not the player was a human or an AI/CPU player.
    pub fn human(&self) -> bool {
        self.human
    }
    /// The faction selected by the player in this match.
    pub fn faction(&self) -> Faction {
        self.faction
    }
    /// The team the player was assigned to. Currently only head-to-head matchups are supported
    /// (max two teams).
    pub fn team(&self) -> Team {
        self.team
    }
    /// The pbgid of the battlegroup the player selected, or `None` if no battlegroup was selected.
    /// For details on what this ID represents please see `SelectBattlegroup::pbgid`.
    pub fn battlegroup(&self) -> Option<u32> {
        self.battlegroup
    }
    /// The Steam ID of the player, or `None` if the player is AI. This ID can be used to uniquely
    /// identify a player between replays, and connect them to their Steam profile.
    pub fn steam_id(&self) -> Option<u64> {
        self.steam_id
    }
    /// The Relic profile ID of the player, or `None` if the player is AI. This ID can be used to
    /// uniquely identify a player between replays, and can be used to query statistical information
    /// about the player from Relic's stats API.
    pub fn profile_id(&self) -> Option<u64> {
        self.profile_id
    }
    /// A list of all messages sent by the player in the match. Sorted chronologically from first
    /// to last.
    pub fn messages(&self) -> Vec<Message> {
        self.messages.clone()
    }

    /// A list of all commands executed by the player in the match. Sorted chronologically from
    /// first to last.
    pub fn commands(&self) -> Vec<Command> {
        self.commands.clone()
    }

    /// A list of only build-related commands executed by the player in the match. A build command
    /// is any that enqueues the construction of a new unit or upgrade. Sorted chronologically from
    /// first to last.
    pub fn build_commands(&self) -> Vec<Command> {
        self.commands
            .clone()
            .into_iter()
            .filter(|entry| {
                matches!(
                    entry,
                    Command::BuildGlobalUpgrade(_) | Command::BuildSquad(_)
                )
            })
            .collect()
    }

    /// A list of only battlegroup-related commands executed by the player in the match. A
    /// battlegroup command is any that involves the select or use of battlegroups and their
    /// abilities.
    pub fn battlegroup_commands(&self) -> Vec<Command> {
        self.commands
            .clone()
            .into_iter()
            .filter(|entry| {
                matches!(
                    entry,
                    Command::SelectBattlegroup(_)
                        | Command::SelectBattlegroupAbility(_)
                        | Command::UseBattlegroupAbility(_)
                )
            })
            .collect()
    }

    pub fn raw_command_data(&self) -> Vec<Raw> {
        self.raw_command_data.clone()
    }
}

pub(crate) fn player_from_data(player_data: &PlayerData, ticks: Vec<&Tick>) -> Player {
    let mut player = Player {
        name: player_data.name.clone(),
        human: player_data.human != 0,
        faction: Faction::try_from(player_data.faction.as_ref()).unwrap(),
        team: Team::try_from(player_data.team).unwrap(),
        steam_id: None,
        profile_id: None,
        messages: messages_from_data(&ticks, &player_data.name),
        commands: commands_from_data(&ticks, player_data.id),
        battlegroup: None,
        raw_command_data: raw_from_data(&ticks, player_data.id)
    };

    if player.human {
        player.steam_id = Some(str::parse(&player_data.steam_id).unwrap());
        player.profile_id = Some(player_data.profile_id);
    }

    player.battlegroup = match player
        .commands
        .iter()
        .find(|&command| matches!(command, Command::SelectBattlegroup(_)))
    {
        Some(Command::SelectBattlegroup(command)) => Some(command.pbgid()),
        Some(_) => panic!(),
        None => None,
    };

    player
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// this is safe as Player does not contain any Ruby types
#[cfg(feature = "magnus")]
unsafe impl magnus::IntoValueFromNative for Player {}

/// Company of Heroes 3 factions.

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "VaultCoh::Faction"))]
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

/// Representation of a player's team membership.

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "VaultCoh::Team"))]
pub enum Team {
    First = 0,
    Second = 1,
}

impl Team {
    /// Integer representation of the assigned team.
    pub fn value(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<u32> for Team {
    type Error = String;

    fn try_from(input: u32) -> Result<Team, Self::Error> {
        match input {
            0 => Ok(Team::First),
            1 => Ok(Team::Second),
            _ => Err(format!("Invalid team ID {}!", input)),
        }
    }
}
