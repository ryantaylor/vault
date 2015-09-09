use std::path::Path;
use std::string::String;

use stream::{Stream, StreamError};
use player::Player;

pub struct Replay {
    file: Stream,
    version: u16,
    game_type: String,
    date_time: String,
    map_file: String,
    map_name: String,
    map_description: String,
    map_width: u32,
    map_height: u32,
    map_players: u32,
    players: Vec<Player>,
    duration: u32,              // seconds
    rng_seed: u32,
    opponent_type: u32,         // 1 = human, 2 = cpu
}

impl Replay {
    pub fn new(path: &Path) -> Replay {
        Replay {
            file: Stream::new(&path),
            version: 0,
            game_type: String::new(),
            date_time: String::new(),
            map_file: String::new(),
            map_name: String::new(),
            map_description: String::new(),
            map_width: 0,
            map_height: 0,
            map_players: 0,
            players: Vec::new(),
            duration: 0,
            rng_seed: 0,
            opponent_type: 0,
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

        self.display();
    }

    fn parse_version(&mut self) {
        trace!("Replay::parse_version");
        self.version = self.file.read_u16().unwrap();
        if self.version < 19545 {
            panic!("version {} unsupported, minimum version 19545 required");
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

    fn parse_map_info(&mut self) {
        trace!("Replay::parse_map_info");
        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        self.file.skip_ahead(4).unwrap(); // can be 1 or 2?
        assert_eq!(self.file.read_u32().unwrap(), 0x3);
        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        assert_eq!(self.file.read_u32().unwrap(), 0x0);

        let mut size = self.file.read_u32().unwrap();
        self.map_file = self.file.read_utf8(size).unwrap();

        self.file.skip_ahead(16).unwrap(); // something to do with map start positions?

        size = self.file.read_u32().unwrap();
        self.map_name = self.file.read_utf16(size).unwrap();

        assert_eq!(self.file.read_u32().unwrap(), 0x0);

        size = self.file.read_u32().unwrap();
        self.map_description = self.file.read_utf16(size).unwrap();

        self.map_players = self.file.read_u32().unwrap();

        self.map_width = self.file.read_u32().unwrap();
        self.map_height = self.file.read_u32().unwrap();
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
            error!("Replay::parse_chunk - invalid chunk type {} at cursor {}", chunk_type, self.file.get_cursor_position());
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

        if chunk_type == "DATASDSC" && chunk_version == 0x7E4 {
            self.parse_map_info();
        }

        if chunk_type == "DATADATA" && chunk_version == 0x1B {
            self.parse_opponent_info();

            self.file.skip_ahead(4).unwrap(); // 0 or 1
            assert_eq!(self.file.read_u32().unwrap(), 0x0);
            assert_eq!(self.file.read_u16().unwrap(), 0x0);

            self.parse_rng_seed();

            self.parse_players();

            //self.file.skip(90);
        }

        self.file.seek(start_position + chunk_length);

        true
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
        self.file.read_utf8(size).unwrap(); // Seb: default or cpu default_skirmish

        self.file.skip_ahead(4).unwrap(); // Seb: this is not count, it's t1p1 t2p1 t1p2 t2p2 etc (fixed pos)
                                          // or I dont even know anymore (for random) its still count

        self.file.skip_ahead(4).unwrap(); // something (not position)

        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        assert_eq!(self.file.read_u32().unwrap(), 0x5);

        let mut done = false;

        while !done {
            let mut val = self.file.read_u16().unwrap();
            while val == 0x1 {
                val = self.file.read_u16().unwrap();
            }

            // 0x0 means we've parsed all data
            if val == 0x0 {
                done = true;
            }
            // 0x109 means we're at a new data section
            else if val == 0x109 {
                self.file.skip_ahead(4).unwrap();
                assert_eq!(self.file.read_u32().unwrap(), 0x0);
                self.file.skip_ahead(4).unwrap();
                assert_eq!(self.file.read_u32().unwrap(), 0x0);

                let size = self.file.read_u16().unwrap() as u32;
                self.file.skip_ahead(size).unwrap();
            }
            // 0x3 means we're at the end of a data section, so just keep going
            else if val == 0x3 {
                assert_eq!(self.file.read_u16().unwrap(), 0x0);
            }
            // otherwise we found steam id
            else {
                self.file.skip_ahead(2).unwrap();
                assert_eq!(self.file.read_u32().unwrap(), 0x0);
                player.update_steam_id(self.file.read_u64().unwrap());
            }
        }

        assert_eq!(self.file.read_u16().unwrap(), 0x0);
        self.file.skip_ahead(8).unwrap();

        player
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

                    self.file.skip_ahead(bundle_length).unwrap(); // until I add handling
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

    fn display(&self) {
        println!("version: {}", self.version);
        println!("game_type: {}", self.game_type);
        println!("date_time: {}", self.date_time);
        println!("map_file: {}", self.map_file);
        println!("map_name: {}", self.map_name);
        println!("map_description: {}", self.map_description);
        println!("map_width: {}", self.map_width);
        println!("map_height: {}", self.map_height);
        println!("map_players: {}", self.map_players);
        println!("duration: {}", self.duration);
        println!("num players: {}", self.players.len());

        for player in self.players.iter() {
            player.display();
        }
    }
}