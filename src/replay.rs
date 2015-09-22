//! A module containing a representative state of a Company of Heroes 2 replay file.

use std::error::Error as StdError;
use std::path::Path;
use std::string::String;

use rustc_serialize::json;

use chat_line::ChatLine;
use command::Command;
use Error;
use item::{Item, ItemType};
use map::Map;
use player::Player;
use Result;
use stream::Stream;

/// Takes a Result<T>, unwraps it, then checks for equality against another T. If Result<T>
/// unwraps to an Err, that Err is returned. If the equality check fails, an Err is returned
/// instead of panicking.

#[cfg(not(debug_assertions))]
macro_rules! test_eq {
    ($a:expr, $b:expr) => ({
        use std::result::Result;
        let exp = try!($a);
        let (a, b) = (&exp, &$b);
        match *a == *b {
            true => true,
            false => return Result::Err(Error::UnexpectedValue)
        }
    })
}

/// Debug version of the above macro, panics if Result<T> unwraps to Err or the equality check
/// fails.

#[cfg(debug_assertions)]
macro_rules! test_eq {
    ($a:expr, $b:expr) => ({
        let exp = $a.unwrap();
        let (a, b) = (&exp, &$b);
        assert_eq!(*a, *b);
    })
}

/// The main Replay type, contains all currently parsed replay data. Can be serialized to JSON for
/// output using rustc_serialize.

#[derive(Debug, RustcEncodable)]
pub struct Replay {
    error: Option<String>,
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

    /// Constructs a new Replay and loads the file specified by path into memory.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate vault;
    ///
    /// use vault::Replay;
    /// use std::path::Path;
    ///
    /// let path = Path::new("/path/to/replay.rec");
    /// let replay = Replay::new(&path).unwrap();
    /// ```

    pub fn new(path: &Path) -> Result<Replay> {
        Ok(Replay {
            error: None,
            file: try!(Stream::from_file(&path)),
            version: 0,
            game_type: String::new(),
            date_time: String::new(),
            map: Map::new(),
            players: Vec::with_capacity(8),
            duration: 0,
            rng_seed: 0,
            opponent_type: 0,
            chat: Vec::new(),
        })
    }

    /// Constructs a junk Replay type with empty data and an error value set. Used to return a
    /// Replay and its error information out of a thread without panicking if an error was
    /// encountered during creation.
    ///
    /// # Examples
    ///
    ///```
    /// extern crate vault;
    ///
    /// use vault::Replay;
    ///
    /// match Replay::new("/path/to/replay.rec") {
    ///     Ok(replay) => replay,
    ///     Err(err) => Replay::with_new_error(err),
    /// }
    /// ```

    pub fn new_with_error(name: &str, err: Error) -> Replay {
        Replay {
            error: Some(err.description().to_string()),
            file: Stream::new(name),
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

    /// Constructs a new Replay and loads the byte vector given as the file data.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate vault;
    /// extern crate zip;
    ///
    /// use vault::Replay;
    /// use std::path::Path;
    /// use zip::read::{ZipArchive. ZipFile};
    ///
    /// let path = Path::new("/path/to/archive.zip");
    /// let file = File::open(&path).unwrap();
    /// let archive = ZipArchive::new(file).unwrap();
    /// let mut buff: Vec<u8> = Vec::with_capacity(replay_file.size() as usize);
    /// let mut replay_file = archive.by_index(idx).unwrap();
    ///
    /// replay_file.read_to_end(&mut buff).unwrap();
    /// let mut replay = Replay::from_bytes(buff).unwrap();
    /// replay.parse();
    /// ```

    pub fn from_bytes(name: &str, bytes: Vec<u8>) -> Result<Replay> {
        Ok(Replay {
            error: None,
            file: try!(Stream::from_bytes(name, bytes)),
            version: 0,
            game_type: String::new(),
            date_time: String::new(),
            map: Map::new(),
            players: Vec::with_capacity(8),
            duration: 0,
            rng_seed: 0,
            opponent_type: 0,
            chat: Vec::new(),
        })
    }

    /// Parses the loaded replay and populates the Replay type with the return data.
    ///
    /// When the replay has finished being parsed, the vector of byte data loaded into memory from
    /// file is dropped. This is done to clean up the resulting type in order to make working with
    /// the output easier. The file cursor property remains, however, and is an accurate
    /// representation of the size of the replay file in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate vault;
    ///
    /// use vault::replay::Replay;
    /// use std::path::Path;
    ///
    /// let path = Path::new("/path/to/replay.rec");
    /// let replay = Replay::new(&path).unwrap();
    ///
    /// replay.parse();
    /// 
    /// // You can serialize the Replay type to JSON after parsing if you would like to print
    /// // replay data to stdout or write it to a file.
    /// //
    /// let encoded = replay.to_json().unwrap();
    /// println!("{}", encoded);
    /// ```

    pub fn parse(&mut self) {
        match self.parse_version() {
            Ok(_) => (),
            Err(err) => {
                self.update_error(err);
                self.cleanup();
                return;
            }
        }

        match self.parse_game_type() {
            Ok(_) => (),
            Err(err) => {
                self.update_error(err);
                self.cleanup();
                return;
            }
        }

        match self.parse_date_time() {
            Ok(_) => (),
            Err(err) => {
                self.update_error(err);
                self.cleanup();
                return;
            }
        }

        self.file.seek(76);

        match self.parse_chunky() {
            Ok(_) => (),
            Err(err) => {
                self.update_error(err);
                self.cleanup();
                return;
            }
        }

        match self.parse_chunky() {
            Ok(_) => (),
            Err(err) => {
                self.update_error(err);
                self.cleanup();
                return;
            }
        }

        match self.parse_data() {
            Ok(_) => (),
            Err(err) => {
                self.update_error(err);
                self.cleanup();
                return;
            }
        }

        self.cleanup();
    }

    /// Parses a Chunky file segment at the current Stream cursor.
    ///
    /// A Chunky segment is a container of replay data that houses one or more Chunks.

    fn parse_chunky(&mut self) -> Result<()> {
        trace!("Replay::parse_chunky");
        test_eq!(self.file.read_utf8(12), "Relic Chunky"); // chunk name
        test_eq!(self.file.read_u32(), 0x1A0A0D); // chunky type
        test_eq!(self.file.read_u32(), 0x3); // chunky version
        test_eq!(self.file.read_u32(), 0x1);
        test_eq!(self.file.read_u32(), 0x24);
        test_eq!(self.file.read_u32(), 0x1C);
        test_eq!(self.file.read_u32(), 0x1);

        while try!(self.parse_chunk()) {}
        Ok(())
    }

    /// Parses a Chunk file segment at the current Stream cursor.
    ///
    /// A Chunk segment is a block of replay data inside a Chunky segment that contains one or more
    /// pieces of information that we want to parse out. Depending on the Chunk type and Chunk
    /// version, different parsing rules apply and different information is pulled from the file
    /// into the Replay.

    fn parse_chunk(&mut self) -> Result<bool> {
        trace!("Replay::parse_chunk");
        let chunk_type = try!(self.file.read_utf8(8));
        if !chunk_type.starts_with("FOLD") && !chunk_type.starts_with("DATA") {
            error!("Replay::parse_chunk - invalid chunk type {} at cursor {}", 
                   chunk_type, 
                   self.file.get_cursor_position());
            try!(self.file.skip_back(8));
            return Ok(false);
        }

        let chunk_version = try!(self.file.read_u32());
        let chunk_length = try!(self.file.read_u32());
        let chunk_name_length = try!(self.file.read_u32());

        try!(self.file.skip_ahead(4)); // 0, 2000 (dec), or FF..
        test_eq!(self.file.read_u32(), 0x0);

        info!("Replay::parse_chunk - in {} chunk, version {}", chunk_type, chunk_version);
        debug!("Replay::parse_chunk - chunk_version = {}", chunk_version);
        debug!("Replay::parse_chunk - chunk_length = {}", chunk_length);
        debug!("Replay::parse_chunk - chunk_name_length = {}", chunk_name_length);

        let chunk_name: String;
        if chunk_name_length > 0 {
            chunk_name = try!(self.file.read_utf8(chunk_name_length));
            debug!("Replay::parse_chunk - chunk_name = {}", chunk_name);
        }

        let start_position = self.file.get_cursor_position();
        debug!("Replay::parse_chunk - start_position = {}", start_position);

        if chunk_type.starts_with("FOLD") {
            while self.file.get_cursor_position() < start_position + chunk_length {
                try!(self.parse_chunk());
            }
        }

        if chunk_type == "DATASDSC" {
            try!(self.parse_map_info(chunk_version));
        }

        if chunk_type == "DATADATA" {
            try!(self.parse_game_data(chunk_version));
        }

        self.file.seek(start_position + chunk_length);

        Ok(true)
    }

    /// Parses the Data section of a Replay. This section comes after all Chunky and Chunk
    /// segments, and encodes all actions and chat messages.

    fn parse_data(&mut self) -> Result<()> {
        trace!("Replay::parse_data");
        while try!(self.parse_tick()) {}
        Ok(())
    }

    /// Parses a Tick at the current Stream cursor.
    ///
    /// A Tick is a block of data in the Data section of a Replay that stores information about
    /// player actions and chat messages that occurred at that moment in time in the Replay. One
    /// Tick represents 1/8 seconds of real-world time, and can contain Action information (player
    /// commands) or Special information (chat messages).

    fn parse_tick(&mut self) -> Result<bool> {
        trace!("Replay::parse_tick");
        let tick_type = match self.file.read_u32() {
            Err(e) => {
                match e {
                    Error::CursorOutOfBounds => return Ok(false),
                    _ => return Err(e),
                }
            },
            Ok(val) => val,
        };

        let tick_size = match self.file.read_u32() {
            Err(e) => {
                match e {
                    Error::CursorOutOfBounds => return Ok(false),
                    _ => return Err(e),
                }
            },
            Ok(val) => val,
        };

        let start_position = self.file.get_cursor_position();

        if tick_size > 0 {
            // action
            if tick_type == 0x0 {
                try!(self.file.skip_ahead(1)); // usually 0x20 but can be 0x0
                let tick_id = try!(self.file.read_u32());
                let some_id = try!(self.file.read_u32());

                let bundle_count = try!(self.file.read_u32());
                for _ in 0..bundle_count {
                    let bundle_part_count = try!(self.file.read_u32());

                    try!(self.file.skip_ahead(4)); // Seb: thought 0 but can be 33554432

                    let bundle_length = try!(self.file.read_u32());
                    let check = try!(self.file.read_u8()) as u32;

                    if check != bundle_length % 256 {
                        return Err(Error::UnexpectedValue);
                    }
                    //test_eq!(self.file.read_u8() as u32, bundle_length % 256);
                    //test_eq!(try!(self.file.read_u8()) as u32, bundle_length % 256);

                    let mut idx = 0;
                    let mut done = false;

                    while !done {
                        let inter_position = self.file.get_cursor_position();
                        let bundle_part_length = try!(self.file.read_u16()) as u32;

                        try!(self.parse_action(bundle_part_length));

                        let current_position = self.file.get_cursor_position();
                        let diff = inter_position + bundle_part_length - current_position;

                        if diff > 0 {
                            try!(self.file.skip_ahead(diff)); // inter raw
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
                let chat = try!(self.file.read_u32()); // Seb: is chat? most 1 few 0

                if chat == 0x1 {
                    try!(self.file.skip_ahead(4)); // length
                    try!(self.file.skip_ahead(4)); // Seb: chat nbr 2 6 or few 4

                    let mut size = try!(self.file.read_u32());
                    let name = try!(self.file.read_utf16(size));

                    size = try!(self.file.read_u32());
                    let content = try!(self.file.read_utf16(size));

                    info!("{}: {}", name, content);
                    self.chat.push(ChatLine::with_data(self.duration, name, content));

                    let tag_length = try!(self.file.read_u32()); // not sure what this is
                    try!(self.file.skip_ahead(tag_length * 2)); // some numeric ids? all u16s
                }
                else {
                    test_eq!(self.file.read_u32(), 0x8);
                    try!(self.file.skip_ahead(4)); // Seb: special E9 03 00 00 1000 to 1006
                    test_eq!(self.file.read_u32(), 0x0);
                }
            }

            return Ok(true);
        }
        Ok(false)
    }

    /// Parses a segment of an Action type Tick and extracts the details of the contained action,
    /// depending on the type of Command encoded in the action.

    fn parse_action(&mut self, len: u32) -> Result<()> {
        trace!("Replay::parse_action");

        let action_type = try!(self.file.read_u8());
        let base_location = try!(self.file.read_u8());

        try!(self.file.skip_ahead(1)); // part of player ID?
        let player_id = try!(self.file.read_u8());

        try!(self.file.skip_ahead(2)); // probably counts current num of tick_size
        try!(self.file.skip_ahead(2)); // lots of 0, 16 then 20546 2054720802 21085
        try!(self.file.skip_ahead(2)); // pretty sure it's a player id of some sort
        let unit_id = try!(self.file.read_u8()); // unit id

        let command = match Command::from_u8(action_type) {
            Some(val) => val,
            None => return Ok(()),
        };

        match command {
            _ => info!("{}:{}:{:?} u {}", player_id, base_location, command, unit_id)
        }

        Ok(())
    }

    /// Parses the Replay version at the current Stream cursor.

    fn parse_version(&mut self) -> Result<()> {
        trace!("Replay::parse_version");
        test_eq!(self.file.read_u16(), 0x0);
        self.version = try!(self.file.read_u16());
        if self.version < 19545 {
            return Err(Error::UnsupportedVersion);
        }

        Ok(())
    }

    /// Parses the Replay game type at the current Stream cursor.

    fn parse_game_type(&mut self) -> Result<()> {
        trace!("Replay::parse_game_type");
        self.game_type = try!(self.file.read_utf8(8));
        Ok(())
    }

    /// Parses the Replay timestamp at the current Stream cursor.

    fn parse_date_time(&mut self) -> Result<()> {
        trace!("Replay::parse_date_time");
        let mut ch = match self.file.read_utf16_single() {
            Err(e) => {
                match e {
                    Error::EmptyChar => String::new(),
                    _ => return Err(e),
                }
            },
            Ok(val) => val,
        };

        while !ch.is_empty() {
            self.date_time.push_str(&ch);
            ch = match self.file.read_utf16_single() {
                Err(e) => {
                    match e {
                        Error::EmptyChar => String::new(),
                        _ => return Err(e),
                    }
                },
                Ok(val) => val,
            };
        }

        Ok(())
    }

    /// Parses the Replay map information at the current Stream cursor and stores the parsed
    /// information in a Map type associated with the Replay.

    fn parse_map_info(&mut self, version: u32) -> Result<()> {
        trace!("Replay::parse_map_info");
        if version == 0x7E4 {
            test_eq!(self.file.read_u32(), 0x0);
            test_eq!(self.file.read_u32(), 0x0);
            try!(self.file.skip_ahead(4)); // can be 1 or 2?
            test_eq!(self.file.read_u32(), 0x3);
            test_eq!(self.file.read_u32(), 0x0);
            test_eq!(self.file.read_u32(), 0x0);
            test_eq!(self.file.read_u32(), 0x0);

            let mut size = try!(self.file.read_u32());
            let map_file = try!(self.file.read_utf8(size));

            try!(self.file.skip_ahead(16)); // something to do with map start positions?

            size = try!(self.file.read_u32());
            let map_name = try!(self.file.read_utf16(size));

            size = try!(self.file.read_u32());
            let map_description_long = try!(self.file.read_utf16(size));

            size = try!(self.file.read_u32());
            let map_description = try!(self.file.read_utf16(size));

            let map_players = try!(self.file.read_u32());

            let map_width = try!(self.file.read_u32());
            let map_height = try!(self.file.read_u32());

            self.map = Map::with_data(map_file,
                                      map_name,
                                      map_description,
                                      map_description_long,
                                      map_width,
                                      map_height,
                                      map_players);
        }
        else {
            return Err(Error::UnsupportedChunkVersion);
        }

        Ok(())
    }

    /// Parses the Replay opponent information at the current Stream cursor.

    fn parse_opponent_info(&mut self) -> Result<()> {
        trace!("Replay::parse_opponent_info");
        self.opponent_type = try!(self.file.read_u32());
        Ok(())
    }

    /// Parses the Replay RNG seed at the current Stream cursor.

    fn parse_rng_seed(&mut self) -> Result<()> {
        trace!("Replay::parse_rng_seed");
        self.rng_seed = try!(self.file.read_u32());
        Ok(())
    }

    /// Parses a section of game data found in a DATADATA Chunk which includes opponent,
    /// information, RNG seed, and player information.

    fn parse_game_data(&mut self, version: u32) -> Result<()> {
        trace!("Replay::parse_game_data");
        if version == 0x1 {
            return Ok(()); // do nothing
        }
        else if version >= 0x1B && version <= 0x1C {
            try!(self.parse_opponent_info());

            try!(self.file.skip_ahead(4)); // 0 or 1
            test_eq!(self.file.read_u32(), 0x0);
            test_eq!(self.file.read_u16(), 0x0);

            try!(self.parse_rng_seed());

            try!(self.parse_players());
        }
        else {
            return Err(Error::UnsupportedChunkVersion);
        }

        Ok(())
    }

    /// Parses all Player entities in the given Replay, starting at the current Stream cursor.

    fn parse_players(&mut self) -> Result<()> {
        trace!("Replay::parse_players");
        let num_players = try!(self.file.read_u32());
        debug!("Replay::parse_players - {} players found", num_players);

        let mut player: Player;
        for _ in 0..num_players {
            player = try!(self.parse_player());
            self.players.push(player);
        }

        Ok(())
    }

    /// Parses a Player entity at the current Stream cursor, including all Items equipped by that
    /// player.

    fn parse_player(&mut self) -> Result<Player> {
        trace!("Replay::parse_player");
        let mut player = Player::new();

        try!(self.file.skip_ahead(1)); // could be 1 = human player, 0 = cpu player?

        let mut size = try!(self.file.read_u32());
        player.update_name(try!(self.file.read_utf16(size)));
        player.update_team(try!(self.file.read_u32()));

        info!("Replay::parse_player - parsing player {}", player.name());

        size = try!(self.file.read_u32());
        player.update_faction(try!(self.file.read_utf8(size)));
        test_eq!(self.file.read_u32(), 0x5); // 5 for army type

        try!(self.file.skip_ahead(4)); // Seb: p00

        size = try!(self.file.read_u32());
        try!(self.file.read_utf8(size)); // Seb: default or skirmish

        try!(self.file.skip_ahead(4)); // Seb: this is not count, it's t1p1 t2p1 t1p2 t2p2 etc 
                                          // (fixed pos) or I dont even know anymore (for random) 
                                          // its still count

        try!(self.file.skip_ahead(4)); // something (not position)

        test_eq!(self.file.read_u32(), 0x0);
        test_eq!(self.file.read_u32(), 0x5);

        test_eq!(self.file.read_u16(), 0x1); // not sure what this is yet

        player.add_item(try!(self.parse_item(ItemType::Skin)));
        player.add_item(try!(self.parse_item(ItemType::Skin)));
        player.add_item(try!(self.parse_item(ItemType::Skin)));

        test_eq!(self.file.read_u16(), 0x1); // not sure what this is yet

        player.update_steam_id(try!(self.parse_steam_id()));

        player.add_item(try!(self.parse_item(ItemType::FacePlate)));
        player.add_item(try!(self.parse_item(ItemType::VictoryStrike)));
        player.add_item(try!(self.parse_item(ItemType::Decal)));

        size = try!(self.file.read_u32());
        for _ in 0..size {
            player.add_item(try!(self.parse_item(ItemType::Commander)));
        }

        size = try!(self.file.read_u32());
        for _ in 0..size {
            player.add_item(try!(self.parse_item(ItemType::Bulletin)));
        }

        test_eq!(self.file.read_u32(), 0x0);
        try!(self.file.skip_ahead(8)); // don't know what this is yet, 2 u32s

        Ok(player)
    }

    /// Parses an Item belonging to a Player at the current Stream cursor, depending on the type
    /// of Item being parsed, and returns that Item to the caller.

    fn parse_item(&mut self, item_type: ItemType) -> Result<Item> {
        let type_label = try!(self.file.read_u16());
        match type_label {
            0x1 => Ok(Item::new(item_type)),
            0x109 => Ok(try!(self.parse_player_item(item_type))),
            0x206 => Ok(try!(self.parse_cpu_item(item_type))),
            0x216 => Ok(try!(self.parse_player_item_special(item_type))),
            _ => Err(Error::UnexpectedValue),
        }
    }

    /// Parses an encoded Item whose pattern matches that of most human Player Items.

    fn parse_player_item(&mut self, item_type: ItemType) -> Result<Item> {
        let primary = try!(self.file.read_u32());
        test_eq!(self.file.read_u32(), 0x0);
        let secondary = try!(self.file.read_u32());
        test_eq!(self.file.read_u32(), 0x0);

        let size = try!(self.file.read_u16());
        try!(self.file.skip_ahead(size as u32));

        Ok(Item::with_split_id(primary, secondary, item_type))
    }

    /// Parses an encoded Item whose pattern matches that of a special case of human Player Item,
    /// usually a special type of Decal.

    fn parse_player_item_special(&mut self, item_type: ItemType) -> Result<Item> {
        try!(self.file.skip_ahead(16)); // lots of data, no idea what it is
        let id = try!(self.file.read_u32()) as u64; // might not be id
        try!(self.file.skip_ahead(1)); // not sure, was 0x40 in test replay

        Ok(Item::with_whole_id(id, item_type))
    }

    /// Parses an encoded Item whose pattern matches that of most CPU Player Items.

    fn parse_cpu_item(&mut self, item_type: ItemType) -> Result<Item> {
        test_eq!(self.file.read_u8(), 0x1);
        let id = try!(self.file.read_u32()) as u64;

        Ok(Item::with_whole_id(id, item_type))
    }

    /// Parses a Player Steam ID at the current Stream cursor.

    fn parse_steam_id(&mut self) -> Result<u64> {
        try!(self.file.skip_ahead(8)); // u64::MAX if cpu and no steam id, but it will return
                                          // 0 in this case so just read anyways
        Ok(try!(self.file.read_u64()))
    }

    /// Updates the Replay error string to indicate a failure during parsing.

    fn update_error(&mut self, err: Error) {
        self.error = Some(err.description().to_string());
    }

    /// Performs maintenance on the data structures of the Replay type to clean up unneeded
    /// elements once parsing is complete, in order to simplify the resulting information.

    fn cleanup(&mut self) {
        self.file.cleanup();
    }

    /// Serializes Replay as JSON String.

    pub fn to_json(&self) -> Result<String> {
        Ok(try!(json::encode(&self)))
    }

    /// Writes the contents of the Replay to stdout.

    pub fn display(&self) {
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