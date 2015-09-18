use std::path::Path;
use std::string::String;

use stream::{Stream, StreamError};
use player::Player;
use item::{Item, ItemType};
use command::Command;
use map::Map;
use chat_line::ChatLine;

#[derive(Debug, RustcEncodable)]
pub struct Replay {
    file: Stream,
    version: u16,
    game_type: String,
    date_time: String,
    map: Map,
    players: Vec<Player>,
    duration: u32,              // seconds
    rng_seed: u32,
    opponent_type: u32,         // 1 = human, 2 = cpu
    chat: Vec<ChatLine>,
}

impl Replay {
    pub fn new(path: &Path) -> Replay {
        Replay {
            file: Stream::new(&path),
            version: 0,
            game_type: String::new(),
            date_time: String::new(),
            map: Map::new(),
            players: Vec::with_capacity(8),
            duration: 0,
            rng_seed: 0,
            opponent_type: 0,
            chat: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        assert_eq!(self.file.read_u16().unwrap(), 0x0);
        self.parse_version();
        self.parse_game_type();
        self.parse_date_time();

        self.file.seek(76);

        self.parse_chunky();
        self.parse_chunky();

        self.parse_data();

        //self.display();
        self.cleanup();
    }

    fn parse_version(&mut self) {
        trace!("Replay::parse_version");
        self.version = self.file.read_u16().unwrap();
        if self.version < 19545 {
            panic!("version {} unsupported, minimum version 19545 (UKF release) required", self.version);
        }
    }

    fn parse_game_type(&mut self) {
        trace!("Replay::parse_game_type");
        self.game_type = self.file.read_utf8(8).unwrap();
    }

    fn parse_date_time(&mut self) {
        trace!("Replay::parse_date_time");
        let mut ch = match self.file.read_utf16_single() {
            Err(e) => {
                match e {
                    StreamError::EmptyChar => String::new(),
                    _ => panic!("unrecoverable error {:?}", e)
                }
            },
            Ok(val) => val
        };

        while !ch.is_empty() {
            self.date_time.push_str(&ch);
            ch = match self.file.read_utf16_single() {
                Err(e) => {
                    match e {
                        StreamError::EmptyChar => String::new(),
                        _ => panic!("unrecoverable error {:?}", e)
                    }
                },
                Ok(val) => val
            };
        }
    }

    fn parse_map_info(&mut self, version: u32) {
        trace!("Replay::parse_map_info");
        if version == 0x7E4 {
            assert_eq!(self.file.read_u32().unwrap(), 0x0);
            assert_eq!(self.file.read_u32().unwrap(), 0x0);
            self.file.skip_ahead(4).unwrap(); // can be 1 or 2?
            assert_eq!(self.file.read_u32().unwrap(), 0x3);
            assert_eq!(self.file.read_u32().unwrap(), 0x0);
            assert_eq!(self.file.read_u32().unwrap(), 0x0);
            assert_eq!(self.file.read_u32().unwrap(), 0x0);

            let mut size = self.file.read_u32().unwrap();
            let map_file = self.file.read_utf8(size).unwrap();

            self.file.skip_ahead(16).unwrap(); // something to do with map start positions?

            size = self.file.read_u32().unwrap();
            let map_name = self.file.read_utf16(size).unwrap();

            size = self.file.read_u32().unwrap();
            let map_description_long = self.file.read_utf16(size).unwrap();

            size = self.file.read_u32().unwrap();
            let map_description = self.file.read_utf16(size).unwrap();

            let map_players = self.file.read_u32().unwrap();

            let map_width = self.file.read_u32().unwrap();
            let map_height = self.file.read_u32().unwrap();

            self.map = Map::with_data(map_file,
                                      map_name,
                                      map_description,
                                      map_description_long,
                                      map_width,
                                      map_height,
                                      map_players);
        }
        else {
            error!("Replay::parse_game_data - cannot parse DATASDSC version {}", version);
        }
    }

    fn parse_opponent_info(&mut self) {
        trace!("Replay::parse_opponent_info");
        self.opponent_type = self.file.read_u32().unwrap();
    }

    fn parse_rng_seed(&mut self) {
        trace!("Replay::parse_rng_seed");
        self.rng_seed = self.file.read_u32().unwrap();
    }

    fn parse_chunky(&mut self) {
        trace!("Replay::parse_chunky");
        assert_eq!(self.file.read_utf8(12).unwrap(), "Relic Chunky"); // chunk name
        assert_eq!(self.file.read_u32().unwrap(), 0x1A0A0D); // chunky type
        assert_eq!(self.file.read_u32().unwrap(), 0x3); // chunky version
        assert_eq!(self.file.read_u32().unwrap(), 0x1);
        assert_eq!(self.file.read_u32().unwrap(), 0x24);
        assert_eq!(self.file.read_u32().unwrap(), 0x1C);
        assert_eq!(self.file.read_u32().unwrap(), 0x1);

        while self.parse_chunk() {}
    }

    fn parse_chunk(&mut self) -> bool {
        trace!("Replay::parse_chunk");
        let chunk_type = self.file.read_utf8(8).unwrap();
        if !chunk_type.starts_with("FOLD") && !chunk_type.starts_with("DATA") {
            error!("Replay::parse_chunk - invalid chunk type {} at cursor {}", 
                   chunk_type, 
                   self.file.get_cursor_position());
            self.file.skip_back(8).unwrap();
            return false;
        }

        let chunk_version = self.file.read_u32().unwrap();
        let chunk_length = self.file.read_u32().unwrap();
        let chunk_name_length = self.file.read_u32().unwrap();

        self.file.skip_ahead(4).unwrap(); // 0, 2000 (dec), or FF..
        assert_eq!(self.file.read_u32().unwrap(), 0x0);

        info!("Replay::parse_chunk - in {} chunk, version {}", chunk_type, chunk_version);
        debug!("Replay::parse_chunk - chunk_version = {}", chunk_version);
        debug!("Replay::parse_chunk - chunk_length = {}", chunk_length);
        debug!("Replay::parse_chunk - chunk_name_length = {}", chunk_name_length);

        let chunk_name: String;
        if chunk_name_length > 0 {
            chunk_name = self.file.read_utf8(chunk_name_length).unwrap();
            debug!("Replay::parse_chunk - chunk_name = {}", chunk_name);
        }

        let start_position = self.file.get_cursor_position();
        debug!("Replay::parse_chunk - start_position = {}", start_position);

        if chunk_type.starts_with("FOLD") {
            while self.file.get_cursor_position() < start_position + chunk_length {
                self.parse_chunk();
            }
        }

        if chunk_type == "DATASDSC" {
            self.parse_map_info(chunk_version);
        }

        if chunk_type == "DATADATA" {
            self.parse_game_data(chunk_version);
        }

        self.file.seek(start_position + chunk_length);

        true
    }

    fn parse_game_data(&mut self, version: u32) {
        trace!("Replay::parse_game_data");
        if version >= 0x1B && version <= 0x1C {
            self.parse_opponent_info();

            self.file.skip_ahead(4).unwrap(); // 0 or 1
            assert_eq!(self.file.read_u32().unwrap(), 0x0);
            assert_eq!(self.file.read_u16().unwrap(), 0x0);

            self.parse_rng_seed();

            self.parse_players();
        }
        else {
            error!("Replay::parse_game_data - cannot parse DATADATA version {}", version);
        }
    }

    fn parse_players(&mut self) {
        trace!("Replay::parse_players");
        let num_players = self.file.read_u32().unwrap();
        debug!("Replay::parse_players - {} players found", num_players);

        let mut player: Player;
        for _ in 0..num_players {
            player = self.parse_player();
            self.players.push(player);
        }
    }

    fn parse_player(&mut self) -> Player {
        trace!("Replay::parse_player");
        let mut player = Player::new();

        self.file.skip_ahead(1).unwrap(); // could be 1 = human player, 0 = cpu player?

        let mut size = self.file.read_u32().unwrap();
        player.update_name(self.file.read_utf16(size).unwrap());
        player.update_team(self.file.read_u32().unwrap());

        info!("Replay::parse_player - parsing player {}", player.name());

        size = self.file.read_u32().unwrap();
        player.update_faction(self.file.read_utf8(size).unwrap());
        assert_eq!(self.file.read_u32().unwrap(), 0x5); // 5 for army type

        self.file.skip_ahead(4).unwrap(); // Seb: p00

        size = self.file.read_u32().unwrap();
        self.file.read_utf8(size).unwrap(); // Seb: default or skirmish

        self.file.skip_ahead(4).unwrap(); // Seb: this is not count, it's t1p1 t2p1 t1p2 t2p2 etc 
                                          // (fixed pos) or I dont even know anymore (for random) 
                                          // its still count

        self.file.skip_ahead(4).unwrap(); // something (not position)

        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        assert_eq!(self.file.read_u32().unwrap(), 0x5);

        assert_eq!(self.file.read_u16().unwrap(), 0x1); // not sure what this is yet

        player.add_item(self.parse_item(ItemType::Skin));
        player.add_item(self.parse_item(ItemType::Skin));
        player.add_item(self.parse_item(ItemType::Skin));

        assert_eq!(self.file.read_u16().unwrap(), 0x1); // not sure what this is yet

        player.update_steam_id(self.parse_steam_id());

        player.add_item(self.parse_item(ItemType::FacePlate));
        player.add_item(self.parse_item(ItemType::VictoryStrike));
        player.add_item(self.parse_item(ItemType::Decal));

        size = self.file.read_u32().unwrap();
        for _ in 0..size {
            player.add_item(self.parse_item(ItemType::Commander));
        }

        size = self.file.read_u32().unwrap();
        for _ in 0..size {
            player.add_item(self.parse_item(ItemType::Bulletin));
        }

        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        self.file.skip_ahead(8).unwrap(); // don't know what this is yet, 2 u32s

        player
    }

    fn parse_item(&mut self, item_type: ItemType) -> Item {
        let type_label = self.file.read_u16().unwrap();
        match type_label {
            0x1 => Item::new(item_type),
            0x109 => self.parse_player_item(item_type),
            0x206 => self.parse_cpu_item(item_type),
            0x216 => self.parse_player_item_special(item_type),
            _ => panic!("unexpected item type {} at {}", type_label, self.file.get_cursor_position()),
        }
    }

    fn parse_player_item(&mut self, item_type: ItemType) -> Item {
        let primary = self.file.read_u32().unwrap();
        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        let secondary = self.file.read_u32().unwrap();
        assert_eq!(self.file.read_u32().unwrap(), 0x0);

        let size = self.file.read_u16().unwrap();
        self.file.skip_ahead(size as u32).unwrap();

        Item::with_split_id(primary, secondary, item_type)
    }

    fn parse_player_item_special(&mut self, item_type: ItemType) -> Item {
        self.file.skip_ahead(16).unwrap(); // lots of data, no idea what it is
        let id = self.file.read_u32().unwrap() as u64; // might not be id
        self.file.skip_ahead(1).unwrap(); // not sure, was 0x40 in test replay

        Item::with_whole_id(id, item_type)
    }

    fn parse_cpu_item(&mut self, item_type: ItemType) -> Item {
        assert_eq!(self.file.read_u8().unwrap(), 0x1);
        let id = self.file.read_u32().unwrap() as u64;

        Item::with_whole_id(id, item_type)
    }

    fn parse_steam_id(&mut self) -> u64 {
        self.file.skip_ahead(8).unwrap(); // u64::MAX if cpu and no steam id, but it will return
                                          // 0 in this case so just read anyways
        self.file.read_u64().unwrap()
    }

    fn parse_data(&mut self) {
        trace!("Replay::parse_data");
        while self.parse_tick() {}
    }

    fn parse_tick(&mut self) -> bool {
        trace!("Replay::parse_tick");
        let tick_type = match self.file.read_u32() {
            Err(e) => {
                match e {
                    StreamError::CursorOutOfBounds => return false,
                    _ => panic!("unrecoverable error {:?}", e)
                }
            },
            Ok(val) => val
        };

        let tick_size = match self.file.read_u32() {
            Err(e) => {
                match e {
                    StreamError::CursorOutOfBounds => return false,
                    _ => panic!("unrecoverable error {:?}", e)
                }
            },
            Ok(val) => val
        };

        let start_position = self.file.get_cursor_position();

        if tick_size > 0 {
            // action
            if tick_type == 0x0 {
                self.file.skip_ahead(1).unwrap(); // usually 0x20 but can be 0x0
                let tick_id = self.file.read_u32().unwrap();
                let some_id = self.file.read_u32().unwrap();

                let bundle_count = self.file.read_u32().unwrap();
                for _ in 0..bundle_count {
                    let bundle_part_count = self.file.read_u32().unwrap();

                    self.file.skip_ahead(4).unwrap(); // Seb: thought 0 but can be 33554432

                    let bundle_length = self.file.read_u32().unwrap();
                    assert_eq!(self.file.read_u8().unwrap() as u32, bundle_length % 256);

                    let mut idx = 0;
                    let mut done = false;

                    while !done {
                        let inter_position = self.file.get_cursor_position();
                        let bundle_part_length = self.file.read_u16().unwrap() as u32;

                        self.parse_action(bundle_part_length);

                        let current_position = self.file.get_cursor_position();
                        let diff = inter_position + bundle_part_length - current_position;

                        if diff > 0 {
                            self.file.skip_ahead(diff).unwrap(); // inter raw
                        }

                        idx += bundle_part_length;
                        if idx == bundle_length {
                            done = true;
                        }
                    }

                    //self.file.skip_ahead(bundle_length).unwrap(); // until I add handling
                }

                self.duration += 1;
            }
            // special
            else if tick_type == 0x1 {
                let chat = self.file.read_u32().unwrap(); // Seb: is chat? most 1 few 0

                if chat == 0x1 {
                    self.file.skip_ahead(4).unwrap(); // length
                    self.file.skip_ahead(4).unwrap(); // Seb: chat nbr 2 6 or few 4

                    let mut size = self.file.read_u32().unwrap();
                    let name = self.file.read_utf16(size).unwrap();

                    size = self.file.read_u32().unwrap();
                    let content = self.file.read_utf16(size).unwrap();

                    info!("{}: {}", name, content);
                    self.chat.push(ChatLine::with_data(self.duration, name, content));

                    let tag_length = self.file.read_u32().unwrap(); // not sure what this is
                    self.file.skip_ahead(tag_length * 2).unwrap(); // some numeric ids? all u16s
                }
                else {
                    assert_eq!(self.file.read_u32().unwrap(), 0x8);
                    self.file.skip_ahead(4).unwrap(); // Seb: special E9 03 00 00 1000 to 1006
                    assert_eq!(self.file.read_u32().unwrap(), 0x0);
                }
            }

            return true;
        }
        false
    }

    fn parse_action(&mut self, len: u32) {
        trace!("Replay::parse_action");

        let action_type = self.file.read_u8().unwrap();
        let base_location = self.file.read_u8().unwrap();

        self.file.skip_ahead(1).unwrap(); // part of player ID?
        let player_id = self.file.read_u8().unwrap();

        self.file.skip_ahead(2).unwrap(); // probably counts current num of tick_size
        self.file.skip_ahead(2).unwrap(); // lots of 0, 16 then 20546 2054720802 21085
        self.file.skip_ahead(2).unwrap(); // pretty sure it's a player id of some sort
        let unit_id = self.file.read_u8().unwrap(); // unit id

        let command = match Command::from_u8(action_type) {
            Some(val) => val,
            None => {
                //error!("unknown command {}", action_type);
                return;
            }
        };

        match command {
            _ => info!("{}:{}:{:?} u {}", player_id, base_location, command, unit_id)
        }
    }

    fn cleanup(&mut self) {
        self.file.cleanup();
    }

    fn display(&self) {
        println!("version: {}", self.version);
        println!("game_type: {}", self.game_type);
        println!("date_time: {}", self.date_time);
        self.map.display();
        println!("duration: {}", self.duration);
        println!("num players: {}", self.players.len());

        for player in self.players.iter() {
            player.display();
        }

        for line in self.chat.iter() {
            line.display();
        }
    }
}