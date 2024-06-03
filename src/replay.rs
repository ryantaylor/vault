//! Representation of parsed replay information.

use crate::data::chunks::DataAutoChunk;
use crate::data::{Replay as ReplayData, Span};
use crate::map::{map_from_data, Map};
use crate::player::{player_from_data, Player};
use crate::ParseError;
use nom_locate::LocatedSpan;
use nom_tracable::TracableInfo;
use std::fmt;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A complete representation of all information able to be parsed from a Company of Heroes 3
/// replay. Note that parsing is not yet exhaustive, and iterative improvements will be made to
/// pull more information from replay files over time.

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "VaultCoh::Replay"))]
pub struct Replay {
    version: u16,
    timestamp: String,
    game_type: GameType,
    matchhistory_id: Option<u64>,
    mod_uuid: Uuid,
    map: Map,
    players: Vec<Player>,
    length: usize,
}

impl Replay {
    /// Takes a byte slice, parses it as a CoH3 replay, and returns a representation of the parsed
    /// information. Any failures during parsing or conversion will return an error.
    ///
    /// ```ignore
    /// fn main() {
    ///     let data = include_bytes!("/path/to/replay.rec");
    ///     let replay = vault::Replay::from_bytes(data);
    ///     assert!(replay.is_ok())
    /// }
    /// ```
    pub fn from_bytes(input: &[u8]) -> Result<Replay, ParseError> {
        let info = TracableInfo::new().parser_width(64).fold("term");
        let input: Span = LocatedSpan::new_extra(input, info);
        let (_, replay) = ReplayData::from_span(input)?;
        Ok(replay_from_data(&replay))
    }

    /// The Company of Heroes 3 game version this replay was recorded on. Note that this is probably
    /// more accurated described as the build version, and represents the final segment of digits
    /// you see in the game version on the game's main menu. Every time the game is patched, this
    /// version will change, and replays are generally only viewable on the same game version they
    /// were recorded on.
    pub fn version(&self) -> u16 {
        self.version
    }
    /// A UTF-16 representation of the recording user's local time when the replay was recorded.
    /// Note that value may contain non-standard characters and is not guaranteed to be parsable
    /// into an accurate date/time format.
    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
    /// The type of game this replay represents. Note that this information is parsed on a best-
    /// effort basis and therefore may not always be correct. Also note that it's currently not
    /// known if there's a way to differentiate between automatch and custom games for replays
    /// recorded before the replay system release in patch 1.4.0. Games played before that patch
    /// will be marked as either `Skirmish` (for local AI games) or `Multiplayer` (for networked
    /// custom or automatch games). Games recorded on or after patch 1.4.0 will properly
    /// differentiate between `Custom` and `Automatch` games.
    pub fn game_type(&self) -> GameType {
        self.game_type
    }
    /// The ID used by Relic to track this match on their internal servers. This ID can be matched
    /// with an ID of the same name returned by Relic's CoH3 stats API, enabling linkage between
    /// replay files and statistical information for a match. When the game type is `Skirmish`,
    /// there is no ID assigned by Relic, so this will be `None`.
    pub fn matchhistory_id(&self) -> Option<u64> {
        self.matchhistory_id
    }
    /// The UUID of the base game mod this replay ran on. If no mod was used, this will be a nil
    /// UUID (all zeroes).
    pub fn mod_uuid(&self) -> Uuid {
        self.mod_uuid
    }
    /// Map information for this match.
    pub fn map(&self) -> Map {
        self.map.clone()
    }
    /// Filename of the map this match was played on. See `Map::filename` for more information.
    pub fn map_filename(&self) -> &str {
        self.map.filename()
    }
    /// Localization ID of the map's name. See `Map::localized_name_id` for more information.
    pub fn map_localized_name_id(&self) -> &str {
        self.map.localized_name_id()
    }
    /// Localization ID of the map's description. See `Map::localized_description_id` for more
    /// information.
    pub fn map_localized_description_id(&self) -> &str {
        self.map.localized_description_id()
    }
    /// A list of all players who participated in this match.
    pub fn players(&self) -> Vec<Player> {
        self.players.clone()
    }
    /// A simple count of the number of ticks that were executed in this match. Because CoH3's
    /// engine runs at 8 ticks per second, you can divide this value by 8 to get the duration of
    /// the match in seconds.
    pub fn length(&self) -> usize {
        self.length
    }
}

fn replay_from_data(data: &ReplayData) -> Replay {
    let commands = data.commands();
    let messages = data.messages();
    #[cfg(feature = "raw")]
    let raw_commands = data.raw_commands();

    Replay {
        version: data.header.version,
        timestamp: data.header.timestamp.clone(),
        game_type: game_type_from_data(data),
        matchhistory_id: matchhistory_id_from_data(data),
        mod_uuid: data.game_data().mod_uuid,
        map: map_from_data(data.map_data()),
        length: data.command_ticks().count(),
        players: data
            .game_data()
            .players
            .iter()
            .map(|player| {
                player_from_data(
                    player,
                    &messages,
                    &commands,
                    #[cfg(feature = "raw")]
                    &raw_commands,
                )
            })
            .collect(),
    }
}

fn matchhistory_id_from_data(data: &ReplayData) -> Option<u64> {
    if game_type_from_data(data) == GameType::Skirmish {
        None
    } else {
        Some(data.game_data().matchhistory_id)
    }
}

/// Company of Heroes 3 game types

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "VaultCoh::GameType"))]
pub enum GameType {
    /// Local games against AI opponents
    Skirmish,
    /// Networked games that couldn't be more specifically defined; includes both custom and
    /// automatch games from before patch 1.4.0
    Multiplayer,
    /// Ranked automatch games, detectable post patch 1.4.0
    Automatch,
    /// Custom games against human opponents, AI opponents, or a mix of both, detectable post patch
    /// 1.4.0
    Custom,
}

impl Display for GameType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            GameType::Skirmish => write!(f, "skirmish"),
            GameType::Multiplayer => write!(f, "multiplayer"),
            GameType::Automatch => write!(f, "automatch"),
            GameType::Custom => write!(f, "custom"),
        }
    }
}

fn game_type_from_data(data: &ReplayData) -> GameType {
    if data.game_data().skirmish {
        GameType::Skirmish
    } else {
        match data.automatch_data() {
            Some(DataAutoChunk { automatch: true }) => GameType::Automatch,
            Some(DataAutoChunk { automatch: false }) => GameType::Custom,
            None => GameType::Multiplayer,
        }
    }
}
