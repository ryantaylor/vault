use std::string::String;

pub struct Map {
    file: String,
    name: String,
    description: String,
    description_long: String,
    width: u32,
    height: u32,
    players: u32
}

impl Map {
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