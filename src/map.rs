//! A module containing a representation of a map in CoH2 replays.

use std::string::String;

/// This type represents a Company of Heroes 2 map as presented in a CoH2 replay file.

#[derive(Debug, RustcEncodable)]
pub struct Map {
    pub file: String,
    pub name: String,
    pub description: String,
    pub description_long: String,
    pub width: u32,
    pub height: u32,
    pub players: u32
}

impl Map {

    /// Constructs a new Map with empty initial data.

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

    /// Constructs a new Map initialized with the data given.

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

    /// Writes the contents of the Map to stdout.

    pub fn display(&self) {
        println!("map_file: {}", self.file);
        println!("map_name: {}", self.name);
        println!("map_description: {}", self.description);
        println!("map_description_long: {}", self.description_long);
        println!("map_width: {}", self.width);
        println!("map_height: {}", self.height);
        println!("map_players: {}", self.players);
    }
}