//! A module containing a representation of a map in CoH2 replays.

use std::string::String;

/// This type represents a Company of Heroes 2 map as presented in a CoH2 replay file.

#[derive(Debug, RustcEncodable)]
pub struct Map {
    /// Internal Relic map file path
    pub file: String,
    /// Locale string representation of map name
    pub name: String,
    /// Locale string representation of map description
    pub description: String,
    /// If applicable, alternate text map description
    pub description_long: String,
    /// Map width in game units
    pub width: u32,
    /// Map height in game units
    pub height: u32,
    /// Number of players supported by the map
    pub players: u32,
}

impl Map {

    /// Constructs a new `Map` with empty initial data.

    pub fn new() -> Map {
        Map {
            file: String::new(),
            name: String::new(),
            description: String::new(),
            description_long: String::new(),
            width: 0,
            height: 0,
            players: 0
        }
    }

    /// Constructs a new `Map` initialized with the data given.

    pub fn with_data(file: String,
                     name: String,
                     description: String,
                     description_long: String,
                     width: u32,
                     height: u32,
                     players: u32) -> Map {
        Map {
            file: file,
            name: name,
            description: description,
            description_long: description_long,
            width: width,
            height: height,
            players: players
        }
    }
}