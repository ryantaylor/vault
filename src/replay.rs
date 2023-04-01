//! Representation of parsed replay information.

use crate::data::{Replay as ReplayData, Span};
use crate::map::{map_from_data, Map};
use crate::player::{player_from_data, Player};
use crate::ParseError;
use nom_locate::LocatedSpan;
use nom_tracable::TracableInfo;

/// A complete representation of all information able to be parsed from a Company of Heroes 3
/// replay. Note that parsing is not yet exhaustive, and iterative improvements will be made to
/// pull more information from replay files over time.

#[derive(Debug)]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "Vault::Replay"))]
pub struct Replay {
    version: u16,
    timestamp: String,
    matchhistory_id: u64,
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
    /// The ID used by Relic to track this match on their internal servers. This ID can be matched
    /// with an ID of the same name returned by Relic's CoH3 stats API, enabling linkage between
    /// replay files and statistical information for a match.
    pub fn matchhistory_id(&self) -> u64 {
        self.matchhistory_id
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
