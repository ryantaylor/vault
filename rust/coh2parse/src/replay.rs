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
            players: Vec::new(),
            duration: 0,
            rng_seed: 0,
            opponent_type: 0,
        }
    }

    pub fn parse(&mut self) {
        assert_eq!(self.file.read_u16().unwrap(), 0);
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
        assert_eq!(self.file.read_u32().unwrap(), 0);
        assert_eq!(self.file.read_u32().unwrap(), 0);
        self.file.skip_ahead(4).unwrap(); // can be 1 or 2?
        assert_eq!(self.file.read_u32().unwrap(), 3);
        assert_eq!(self.file.read_u32().unwrap(), 0);
        assert_eq!(self.file.read_u32().unwrap(), 0);
        assert_eq!(self.file.read_u32().unwrap(), 0);

        let mut size = self.file.read_u32().unwrap();
        self.map_file = self.file.read_utf8(size).unwrap();

        self.file.skip_ahead(16).unwrap(); // something to do with map start positions?

        size = self.file.read_u32().unwrap();
        self.map_name = self.file.read_utf16(size).unwrap();

        assert_eq!(self.file.read_u32().unwrap(), 0);

        size = self.file.read_u32().unwrap();
        self.map_description = self.file.read_utf16(size).unwrap();

        assert_eq!(self.file.read_u32().unwrap(), 2);

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
        assert_eq!(self.file.read_u32().unwrap(), 1706509); // chunky type
        assert_eq!(self.file.read_u32().unwrap(), 3); // chunky version
        assert_eq!(self.file.read_u32().unwrap(), 1);
        assert_eq!(self.file.read_u32().unwrap(), 36);
        assert_eq!(self.file.read_u32().unwrap(), 28);
        assert_eq!(self.file.read_u32().unwrap(), 1);

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
        assert_eq!(self.file.read_u32().unwrap(), 0);

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

        if chunk_type == "DATASDSC" && chunk_version == 0x7e4 {
            self.parse_map_info();
        }

        if chunk_type == "DATADATA" && chunk_version == 0x1b {
            self.parse_opponent_info();

            self.file.skip_ahead(4).unwrap(); // 0 or 1
            assert_eq!(self.file.read_u32().unwrap(), 0);
            assert_eq!(self.file.read_u16().unwrap(), 0);

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

        size = self.file.read_u32().unwrap();
        player.update_faction(self.file.read_utf8(size).unwrap());
        assert_eq!(self.file.read_u32().unwrap(), 5); // 5 for army type

        self.file.skip_ahead(4).unwrap(); // Seb: p00

        size = self.file.read_u32().unwrap();
        self.file.read_utf8(size).unwrap(); // Seb: default or cpu default_skirmish

        self.file.skip_ahead(4).unwrap(); // Seb: this is not count, it's t1p1 t2p1 t1p2 t2p2 etc (fixed pos)
                                          // or I dont even know anymore (for random) its still count

        self.file.skip_ahead(4).unwrap(); // something (not position)

        assert_eq!(self.file.read_u32().unwrap(), 0);
        assert_eq!(self.file.read_u32().unwrap(), 5);
        assert_eq!(self.file.read_u16().unwrap(), 1);

        if self.file.read_u16().unwrap() == 0x109 {
            self.parse_player_data(&mut player);
        }

        assert_eq!(self.file.read_u16().unwrap(), 0x0);
        self.file.skip_ahead(8).unwrap();

        //self.file.skip_ahead(45).unwrap();

        //player.update_steam_id(self.file.read_u64().unwrap());

        //self.file.skip_ahead(254).unwrap();

        /*self.file.skip_ahead(4).unwrap(); // Seb: thought FFF but can be 1835264
        self.file.skip_ahead(4).unwrap(); // Seb: some low value int or FFF
        self.file.skip_ahead(4).unwrap(); // Seb: 26 00 00 00 FFFF or in 20, 30, or 40
        self.file.skip_ahead(4).unwrap(); // Seb: 2A 00 00 00 FFFF or in 20, 30, or 40
        self.file.skip_ahead(4).unwrap(); // Seb: thought FFF but can be 1701081711
        self.file.skip_ahead(4).unwrap(); // Seb: low or mid val int or might be 2 unit16 to test
        self.file.skip_ahead(4).unwrap(); // Seb: often 0, or FFFF..

        player.update_steam_id(self.file.read_u64().unwrap());
        player.update_victory_strike(self.file.read_u32().unwrap());

        for _ in 0..3 {
            player.add_commander(self.file.read_u32().unwrap());
        }

        let mut num_bulletins = self.file.read_u32().unwrap();

        for _ in 0..num_bulletins {
            player.add_bulletin(self.file.read_u32().unwrap());
        }

        self.file.skip_ahead(4).unwrap(); // Seb: no idea nbr FFFF or some rather low value int

        num_bulletins = self.file.read_u32().unwrap();

        for _ in 0..num_bulletins {
            size = self.file.read_u32().unwrap();
            player.add_bulletin_name(self.file.read_utf8(size).unwrap());
            assert_eq!(self.file.read_u32().unwrap(), 6);
            /*if idx < num_bulletins {
                size = self.file.read_u32() as i32;
                player.bulletin_names.push(self.file.read_text(size));
            }
            else {
                player.bulletin_names.push(String::new());
            }*/
        }

        self.file.skip_ahead(4).unwrap(); // Seb: lots of 0 or a different nbr
        self.file.skip_ahead(4).unwrap(); // Seb: new value wfa
        self.file.skip_ahead(1).unwrap(); // Seb: no idea nbr end 2 : 0 or 1 ?*/

        player
    }

    fn parse_player_data(&mut self, player: &mut Player) {
        trace!("Replay::parse_player_data");
        info!("parsing data for player {}", player.name());
        self.file.skip_ahead(4).unwrap();
        assert_eq!(self.file.read_u32().unwrap(), 0x0);
        self.file.skip_ahead(4).unwrap();
        assert_eq!(self.file.read_u32().unwrap(), 0x0);

        let size = self.file.read_u16().unwrap() as u32;
        self.file.skip_ahead(size).unwrap();

        let mut val = self.file.read_u16().unwrap();

        // 0x1 means keep reading u16s
        while val == 0x1 {
            val = self.file.read_u16().unwrap();
        }

        // 0x0 means we've parsed all data
        if val == 0x0 {
            return;
        }
        // 0x109 means we're at a new data section
        else if val == 0x109 {
            self.parse_player_data(player);
        }
        // 0x3 means we're at the end of a data section
        else if val == 0x3 {
            assert_eq!(self.file.read_u16().unwrap(), 0x0);
            assert_eq!(self.file.read_u16().unwrap(), 0x109);
            self.parse_player_data(player);
        }
        // otherwise we found steam id
        else {
            self.file.skip_ahead(2).unwrap();
            assert_eq!(self.file.read_u32().unwrap(), 0x0);
            player.update_steam_id(self.file.read_u64().unwrap());

            val = self.file.read_u16().unwrap();
            while val == 0x1 {
                val = self.file.read_u16().unwrap();
            }

            if val == 0x109 {
                self.parse_player_data(player);
            }
        }
    }

    fn parse_data(&mut self) {
        trace!("Replay::parse_data");
        while self.parse_tick() {}
    }

    fn parse_tick(&mut self) -> bool {
        trace!("Replay::parse_tick");
        //if self.file.cursor as usize >= self.file.data.len() {
        //    return false;
        //}

        self.file.skip_ahead(4).unwrap();
        let tick_size = match self.file.read_u32() {
            Err(e) => {
                match e {
                    StreamError::CursorOutOfBounds => return false,
                    _ => panic!("unrecoverable error {:?}", e)
                }
            },
            Ok(val) => val
        };

        if tick_size > 0 {
            self.file.skip_ahead(tick_size).unwrap();
            self.duration += 1;
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
        println!("duration: {}", self.duration);
        println!("num players: {}", self.players.len());

        for player in self.players.iter() {
            player.display();
        }
    }
}