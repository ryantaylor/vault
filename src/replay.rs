//! A module containing a representative state of a Company of Heroes 2 replay file.

use std::collections::HashMap;
use std::error::Error as StdError;
use std::path::Path;
use std::string::String;

use chat_line::ChatLine;
use command::{CmdType, Command};
use config::Config;
use error::Error;
use item::{Item, ItemType};
use map::Map;
use player::Player;
use Result;
use stream::Stream;

/// Takes a `Result<T>`, unwraps it, then checks for equality against another `T`. If `Result<T>`
/// unwraps to an `Err`, that `Err` is returned. If the equality check fails, an `Err` is returned
/// instead of panicking.

#[cfg(not(debug_assertions))]
macro_rules! test_eq {
    ($a:expr, $b:expr) => ({
        use std::result::Result;
        let exp = try!($a);
        let (a, b) = (&exp, &$b);
        if *a == *b {
            true
        } else {
            return Result::Err(Error::UnexpectedValue);
        }
    })
}

/// Debug version of the above macro, panics if `Result<T>` unwraps to `Err` or the equality check
/// fails.

#[cfg(debug_assertions)]
macro_rules! test_eq {
    ($a:expr, $b:expr) => ({
        let exp = $a.unwrap();
        let (a, b) = (&exp, &$b);
        assert_eq!(*a, *b);
    })
}

/// The main `Replay` type, contains all currently parsed replay data. Can be serialized to JSON
/// for output using `rustc_serialize`.

#[derive(Debug, RustcEncodable)]
pub struct Replay {
    /// If parsing failed, this will contain an error string
    pub error: Option<String>,
    /// Abstraction of raw replay file bytes
    file: Stream,
    /// Game engine version
    pub version: u16,
    /// Game type (i.e. Automatch/Annihilate)
    pub game_type: String,
    /// Timestamp encoded in replay file
    pub date_time: String,
    /// Map this game was played on
    pub map: Map,
    /// Players involved in the game
    pub players: Vec<Player>,
    /// Commands issued per player
    pub commands: HashMap<u8, Vec<Command>>,
    /// Game length in seconds
    pub duration: u32,
    /// Internal RNG seed used by game engine
    pub rng_seed: u32,
    /// Type of opponent (1 = human, 2 = cpu)
    pub opponent_type: u32,
    /// List of chat messages sent during replay
    pub chat: Vec<ChatLine>,
    /// Configuration settings used for parsing
    config: Config,
}

impl Replay {

    /// Constructs a new `Replay` and loads the file specified by path into memory.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// extern crate vault;
    ///
    /// use vault::Replay;
    /// use vault::Config;
    /// use std::path::Path;
    ///
    /// let path = Path::new("/path/to/replay.rec");
    /// let config = Config::default();
    /// let replay = Replay::new(&path, config).unwrap();
    /// ```

    pub fn new(path: &Path, config: Config) -> Result<Replay> {
        Ok(Replay {
            error: None,
            file: try!(Stream::from_file(&path)),
            version: 0,
            game_type: String::new(),
            date_time: String::new(),
            map: Map::new(),
            players: Vec::with_capacity(8),
            commands: HashMap::new(),
            duration: 0,
            rng_seed: 0,
            opponent_type: 0,
            chat: Vec::new(),
            config: config,
        })
    }

    /// Constructs a junk `Replay` type with empty data and an error value set. Used to return a
    /// `Replay` and its error information out of a thread without panicking if an error was
    /// encountered during creation.
    ///
    /// # Examples
    ///
    /// ```ignore
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
            error: Some(err.description().to_owned()),
            file: Stream::new(name),
            version: 0,
            game_type: String::new(),
            date_time: String::new(),
            map: Map::new(),
            players: Vec::with_capacity(8),
            commands: HashMap::new(),
            duration: 0,
            rng_seed: 0,
            opponent_type: 0,
            chat: Vec::new(),
            config: Config::default(),
        }
    }

    /// Constructs a new `Replay` and loads the byte vector given as the file data.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// extern crate vault;
    /// extern crate zip;
    ///
    /// use vault::Replay;
    /// use vault::Config;
    /// use std::ops::Deref;
    /// use std::path::Path;
    /// use zip::read::{ZipArchive, ZipFile};
    ///
    /// let path = Path::new("/path/to/archive.zip");
    /// let config = Config::default();
    /// let name = path.to_string_lossy();
    /// let name = name.deref();
    /// let file = File::open(&path).unwrap();
    /// let archive = ZipArchive::new(file).unwrap();
    /// let mut buff: Vec<u8> = Vec::with_capacity(replay_file.size() as usize);
    /// let mut replay_file = archive.by_index(0).unwrap();
    ///
    /// replay_file.read_to_end(&mut buff).unwrap();
    /// let mut replay = Replay::from_bytes(&name, buff, config).unwrap();
    /// replay.parse();
    /// ```

    pub fn from_bytes(name: &str, bytes: Vec<u8>, config: Config) -> Result<Replay> {
        Ok(Replay {
            error: None,
            file: try!(Stream::from_bytes(name, bytes)),
            version: 0,
            game_type: String::new(),
            date_time: String::new(),
            map: Map::new(),
            players: Vec::with_capacity(8),
            commands: HashMap::new(),
            duration: 0,
            rng_seed: 0,
            opponent_type: 0,
            chat: Vec::new(),
            config: config,
        })
    }

    /// Parses the loaded replay and populates the `Replay` type with the return data.
    ///
    /// When the replay has finished being parsed, the vector of byte data loaded into memory from
    /// file is dropped. This is done to clean up the resulting type in order to make working with
    /// the output easier. The file cursor property remains, however, and is an accurate
    /// representation of the size of the replay file in bytes.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// extern crate vault;
    ///
    /// use vault::Replay;
    /// use vault::Config;
    /// use std::path::Path;
    ///
    /// let path = Path::new("/path/to/replay.rec");
    /// let config = Config::default();
    /// let replay = Replay::new(&path, config).unwrap();
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
        macro_rules! try_or_clean {
            ($expr:expr, $rep:ident) => (match $expr {
                Ok(_) => (),
                Err(err) => {
                    $rep.cleanup(Some(err));
                    return;
                }
            })
        }

        try_or_clean!(self.parse_version(), self);
        try_or_clean!(self.parse_game_type(), self);
        try_or_clean!(self.parse_date_time(), self);

        self.file.seek(76);

        try_or_clean!(self.parse_chunky(), self);
        try_or_clean!(self.parse_chunky(), self);
        try_or_clean!(self.parse_data(), self);

        self.cleanup(None);
    }

    /// Parses a Chunky file segment at the current `Stream` cursor.
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

    /// Parses a Chunk file segment at the current `Stream` cursor.
    ///
    /// A Chunk segment is a block of replay data inside a Chunky segment that contains one or more
    /// pieces of information that we want to parse out. Depending on the Chunk type and Chunk
    /// version, different parsing rules apply and different information is pulled from the file
    /// into the `Replay`.

    fn parse_chunk(&mut self) -> Result<bool> {
        trace!("Replay::parse_chunk");

        // a Utf8Error here is acceptable because the last read here will always consume random
        // data that could be invalid UTF-8.
        let chunk_type: String = match self.file.read_utf8(8) {
            Ok(val) => val,
            Err(err) => {
                match err {
                    Error::Utf8Error(_) => "invalid".to_owned(),
                    _ => return Err(err)
                }
            }
        };

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

    /// Parses the `Data` section of a Replay. This section comes after all Chunky and Chunk
    /// segments, and encodes all actions and chat messages.

    fn parse_data(&mut self) -> Result<()> {
        trace!("Replay::parse_data");
        while try!(self.parse_tick()) {}
        Ok(())
    }

    /// Parses a Tick at the current `Stream` cursor.
    ///
    /// A Tick is a block of data in the Data section of a `Replay` that stores information about
    /// player actions and chat messages that occurred at that moment in time in the `Replay`. One
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

        //let start_position = self.file.get_cursor_position();

        if tick_size > 0 {
            // action
            if tick_type == 0x0 {
                // skip command parsing if we don't want to do it
                if !self.config.commands {
                    try!(self.file.skip_ahead(tick_size));
                    self.duration += 1;
                    return Ok(true);
                }

                try!(self.file.skip_ahead(1)); // usually 0x20 but can be 0x0
                let tick_id = try!(self.file.read_u32());
                try!(self.file.skip_ahead(4)); // some id

                let bundle_count = try!(self.file.read_u32());
                for _ in 0..bundle_count {
                    try!(self.file.skip_ahead(4)); // bundle part count
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
                        let bundle_part_length = try!(self.file.read_u8()) as u32;

                        try!(self.parse_action(tick_id, bundle_part_length));

                        let current_position = self.file.get_cursor_position();
                        let diff = inter_position + bundle_part_length - current_position;
                        trace!("inter_position: {}", inter_position);
                        trace!("bundle_part_length: {}", bundle_part_length);
                        trace!("current_position: {}", current_position);
                        trace!("diff: {}", diff);

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
    /// depending on the type of `Command` encoded in the action.

    fn parse_action(&mut self, tick: u32, len: u32) -> Result<()> {
        trace!("Replay::parse_action");

        let bytes = if self.config.command_bytes {
            Some(try!(self.file.read_to_vec(len - 1)))
        } else {
            None
        };

        try!(self.file.skip_ahead(1)); // not sure? mostly 0 I think

        let action_type = try!(self.file.read_u8());
        try!(self.file.skip_ahead(1)); // base location

        try!(self.file.skip_ahead(1)); // part of player ID?
        let player_id = try!(self.file.read_u8());

        try!(self.file.skip_ahead(2)); // probably counts current num of tick_size
        try!(self.file.skip_ahead(2)); // lots of 0, 16 then 20546 2054720802 21085
        try!(self.file.skip_ahead(1)); // command type (CMD, PCMD, SCMD)
        try!(self.file.skip_ahead(1)); // some sort of target ID (unit/building/player)
        let command_sub_id = try!(self.file.read_u8());
        //let unit_id = try!(self.file.read_u8()); // unit id

        let command_type = match CmdType::from_u8(action_type) {
            Some(val) => val,
            None => return Ok(()),
        };

        let mut command = Command::new(tick, command_type);
        command.player_id = player_id;
        command.bytes = bytes;

        // command-specific parsing, when necessary
        match command_type {
            CmdType::CMD_BuildSquad |
            CmdType::CMD_Upgrade => {
                if command_sub_id == 0x14 {
                    try!(self.file.skip_ahead(1)); // inner data length
                    test_eq!(self.file.read_u8(), 0x1);
                    command.entity_id = try!(self.file.read_u32());
                    try!(self.file.skip_ahead(2)); // player ID of some sort I think
                    try!(self.file.skip_ahead(3)); // usually 0 I think
                }
            },
            CmdType::SCMD_Upgrade => {
                if command_sub_id == 0x13 {
                    try!(self.file.skip_ahead(1)); // inner data length
                    test_eq!(self.file.read_u8(), 0x1);
                    command.entity_id = try!(self.file.read_u32());
                }
            },
            CmdType::CMD_RallyPoint |
            CmdType::CMD_Move |
            CmdType::SCMD_Move |
            CmdType::SCMD_AttackMove |
            CmdType::SCMD_Unload => {
                try!(self.file.skip_ahead(1)); // inner data length
                match command_sub_id {
                    0x1 |
                    0x1C => {
                        if try!(self.file.read_u8()) == 0x2 {
                            try!(self.parse_coordinates(&mut command));
                        }
                    },
                    0x6 => {
                        //test_eq!(self.file.read_u32(), 0x0);
                        try!(self.file.skip_ahead(4)); // usually 0x0 but sometimes 0x2
                        test_eq!(self.file.read_u8(), 0x2);
                        try!(self.parse_coordinates(&mut command));
                    },
                    _ => {}
                }
            },
            CmdType::PCMD_ConstructStructure => {
                // there are coordinates in here too
                if command_sub_id == 0x19 {
                    try!(self.file.skip_ahead(1)); // inner data length
                    test_eq!(self.file.read_u8(), 0x1);
                    command.entity_id = try!(self.file.read_u32());
                }
            },
            CmdType::PCMD_ConstructFence => {
                // there are coordinates in here too
                if command_sub_id == 0x1A {
                    try!(self.file.skip_ahead(1)); // inner data length
                    test_eq!(self.file.read_u8(), 0x1);
                    command.entity_id = try!(self.file.read_u32());
                }
            },
            CmdType::PCMD_ConstructField => {
                // there are coordinates in here too
                if command_sub_id == 0x1B {
                    try!(self.file.skip_ahead(1)); // inner data length
                    test_eq!(self.file.read_u8(), 0x1);
                    command.entity_id = try!(self.file.read_u32());
                }
            },
            CmdType::PCMD_SetCommander => {
                if command_sub_id == 0x16 {
                    try!(self.file.skip_ahead(1)); // inner data length
                    try!(self.file.skip_ahead(2)); // usually 0x109, part of commander def
                    let selection_id = try!(self.file.read_u32());
                    let server_id = try!(self.set_commander(command.player_id, selection_id));
                    command.entity_id = server_id;
                }
            },
            CmdType::CMD_Ability => {
                match command_sub_id {
                    0x22 => {
                        try!(self.file.skip_ahead(1)); // inner data length
                        try!(self.file.skip_ahead(1)); // usually 0x1
                        command.entity_id = try!(self.file.read_u32());
                        try!(self.file.skip_ahead(1)); // usually 0x0
                    },
                    _ => {}
                }
            },
            CmdType::SCMD_Ability => {
                match command_sub_id {
                    0x22 => {
                        try!(self.file.skip_ahead(1)); // inner data length
                        try!(self.file.skip_ahead(1)); // usually 0x1
                        command.entity_id = try!(self.file.read_u32());
                        try!(self.file.skip_ahead(1)); // usually 0x0
                    },
                    0x23 => {
                        try!(self.file.skip_ahead(1)); // inner data length
                        try!(self.file.skip_ahead(1)); // usually 0x1
                        command.entity_id = try!(self.file.read_u32());
                        try!(self.file.skip_ahead(1)); // usually 0x0
                        if try!(self.file.read_u8()) == 0x2 {
                            try!(self.parse_coordinates(&mut command));
                        }
                    },
                    _ => {}
                }
            },
            CmdType::PCMD_Ability => {
                match command_sub_id {
                    0x23 |
                    0x28 => {
                        try!(self.file.skip_ahead(1)); // inner data length
                        try!(self.file.skip_ahead(1)); // usually 0x1
                        command.entity_id = try!(self.file.read_u32());
                        try!(self.file.skip_ahead(1)); // usually 0x0
                        if try!(self.file.read_u8()) == 0x2 {
                            try!(self.parse_coordinates(&mut command));
                        }
                    },
                    _ => {}
                }
            },
            CmdType::DCMD_DataCommand1 |
            CmdType::DCMD_DataCommand2 => return Ok(()), // don't want to add these
            _ => {}
        }

        self.add_command(player_id, command);
        Ok(())
    }

    /// Parses the `Replay` version at the current `Stream` cursor.

    fn parse_version(&mut self) -> Result<()> {
        trace!("Replay::parse_version");
        test_eq!(self.file.read_u16(), 0x0);
        self.version = try!(self.file.read_u16());
        if self.version < 19545 {
            return Err(Error::UnsupportedVersion);
        }

        Ok(())
    }

    /// Parses the `Replay` game type at the current `Stream` cursor.

    fn parse_game_type(&mut self) -> Result<()> {
        trace!("Replay::parse_game_type");
        self.game_type = try!(self.file.read_utf8(8));
        Ok(())
    }

    /// Parses the `Replay` timestamp at the current `Stream` cursor.

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

    /// Parses the `Replay` map information at the current `Stream` cursor and stores the parsed
    /// information in a `Map` type associated with the `Replay`.

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

    /// Parses the `Replay` opponent information at the current `Stream` cursor.

    fn parse_opponent_info(&mut self) -> Result<()> {
        trace!("Replay::parse_opponent_info");
        self.opponent_type = try!(self.file.read_u32());
        Ok(())
    }

    /// Parses the `Replay` RNG seed at the current `Stream` cursor.

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

    /// Parses all `Player` entities in the given `Replay`, starting at the current `Stream`
    /// cursor.

    fn parse_players(&mut self) -> Result<()> {
        trace!("Replay::parse_players");
        let num_players = try!(self.file.read_u32()) as u8;
        debug!("Replay::parse_players - {} players found", num_players);

        let mut player: Player;
        for _ in 0..num_players {
            player = try!(self.parse_player());
            self.players.push(player);
        }

        Ok(())
    }

    /// Parses a `Player` entity at the current `Stream` cursor, including all `Items` equipped by
    /// that player.

    fn parse_player(&mut self) -> Result<Player> {
        trace!("Replay::parse_player");
        let mut player = Player::new();

        try!(self.file.skip_ahead(1)); // could be 1 = human player, 0 = cpu player?

        let mut size = try!(self.file.read_u32());
        player.name = try!(self.file.read_utf16(size));
        player.team = try!(self.file.read_u32());

        info!("Replay::parse_player - parsing player {}", player.name);

        size = try!(self.file.read_u32());
        player.faction = try!(self.file.read_utf8(size));
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

        player.items.push(try!(self.parse_item(ItemType::Skin)));
        player.items.push(try!(self.parse_item(ItemType::Skin)));
        player.items.push(try!(self.parse_item(ItemType::Skin)));

        test_eq!(self.file.read_u16(), 0x1); // not sure what this is yet

        player.steam_id = try!(self.parse_steam_id());
        player.steam_id_str = player.steam_id.to_string();

        player.items.push(try!(self.parse_item(ItemType::FacePlate)));
        player.items.push(try!(self.parse_item(ItemType::VictoryStrike)));
        player.items.push(try!(self.parse_item(ItemType::Decal)));

        size = try!(self.file.read_u32());
        for _ in 0..size {
            player.items.push(try!(self.parse_item(ItemType::Commander)));
        }

        size = try!(self.file.read_u32());
        for _ in 0..size {
            player.items.push(try!(self.parse_item(ItemType::Bulletin)));
        }

        test_eq!(self.file.read_u32(), 0x0);
        try!(self.file.skip_ahead(8)); // don't know what this is yet, 2 u32s

        Ok(player)
    }

    /// Parses an `Item` belonging to a `Player` at the current `Stream` cursor, depending on the
    /// type of `Item` being parsed, and returns that `Item` to the caller.

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

    /// Parses an encoded `Item` whose pattern matches that of most human `Player` `Items`.

    fn parse_player_item(&mut self, item_type: ItemType) -> Result<Item> {
        let mut item = Item::new(item_type);
        item.selection_id = try!(self.file.read_u32());
        test_eq!(self.file.read_u32(), 0x0);
        item.server_id = try!(self.file.read_u32());
        test_eq!(self.file.read_u32(), 0x0);

        let size = try!(self.file.read_u16());
        try!(self.file.skip_ahead(size as u32));

        Ok(item)
    }

    /// Parses an encoded `Item` whose pattern matches that of a special case of human `Player`
    /// `Item`, usually a special type of Decal.

    fn parse_player_item_special(&mut self, item_type: ItemType) -> Result<Item> {
        try!(self.file.skip_ahead(16)); // lots of data, no idea what it is
        //let id = try!(self.file.read_u32()) as u64; // might not be id
        try!(self.file.skip_ahead(4)); // something to do with custom decals
        try!(self.file.skip_ahead(1)); // not sure, was 0x40 in test replay

        Ok(Item::new(item_type))
    }

    /// Parses an encoded `Item` whose pattern matches that of most CPU `Player` `Items`.

    fn parse_cpu_item(&mut self, item_type: ItemType) -> Result<Item> {
        test_eq!(self.file.read_u8(), 0x1);
        //let id = try!(self.file.read_u32()) as u64;
        try!(self.file.skip_ahead(4)); // gotta figure out what this is

        Ok(Item::new(item_type))
    }

    /// Parses a `Player` Steam ID at the current `Stream` cursor.

    fn parse_steam_id(&mut self) -> Result<u64> {
        try!(self.file.skip_ahead(8)); // u64::MAX if cpu and no steam id, but it will return
                                       // 0 in this case so just read anyways
        Ok(try!(self.file.read_u64()))
    }

    /// Parses x y z coordinates at the current `Stream` cursor and adds them to the given
    /// `Command`.

    fn parse_coordinates(&mut self, command: &mut Command) -> Result<()> {
        command.x = try!(self.file.read_f32());
        command.y = try!(self.file.read_f32());
        command.z = try!(self.file.read_f32());
        Ok(())
    }

    /// Adds a `Command` to the list for the given `Player`.

    fn add_command(&mut self, player_id: u8, command: Command) {
        let commands = self.commands.entry(player_id).or_insert(Vec::new());
        commands.push(command);
    }

    /// Sets the given player's commander based on the commander's `selection_id`. This function
    /// also links the player to its commands via `player_id`, which is set to the player who has a
    /// commander with the given `selection_id` value. Therefore in order for all players to have
    /// their commands link correctly, at least all but one of them must select commanders before
    /// the game completes.

    fn set_commander(&mut self, player_id: u8, selection_id: u32) -> Result<u32> {
        for player in &mut self.players {
            for item in &player.items {
                if item.item_type == ItemType::Commander && item.selection_id == selection_id {
                    if player.id != 0xF { // default id, shouldn't be set yet
                        return Err(Error::UnexpectedValue);
                    }

                    player.id = player_id;
                    player.commander = item.server_id;
                    return Ok(item.server_id);
                }
            }
        }

        // if we can't find the commander we probably have an AI player, should handle this
        Ok(0)
    }

    /// Updates the `Replay` error string to indicate a failure during parsing.

    fn update_error(&mut self, err: Error) {
        self.error = Some(err.description().to_owned());
    }

    /// Performs maintenance on the data structures of the `Replay` type to clean up unneeded
    /// elements once parsing is complete, in order to simplify the resulting information. Also
    /// performs some final transformations to the data, such as calculating CPM and attempting to
    /// link orphaned players to their commands.

    fn cleanup(&mut self, err: Option<Error>) {
        if let Some(val) = err {
            self.update_error(val);
            self.commands = HashMap::new();
        } else {
            let mut unclaimed_player_ids = Vec::new();

            // calculate cpm and check for any players that didn't have an id set during parsing.
            for (player_id, commands) in &self.commands {
                let mut found = false;
                for player in &mut self.players {
                    if player.id == *player_id {
                        player.cpm = commands.len() as f64 / ( self.duration as f64 / 480.0f64 ); // 8 ticks/s x 60s = 480 ticks/min
                        found = true;
                        break;
                    }
                }

                if !found {
                    unclaimed_player_ids.push(player_id);
                }
            }

            // if we have one unclaimed id, we can match it to the only player without a valid id
            // set.
            if unclaimed_player_ids.len() == 1 {
                for player in &mut self.players {
                    if player.id == 0xF { // default junk player id
                        player.id = *unclaimed_player_ids[0];
                        if let Some(commands) = self.commands.get(&player.id) {
                            player.cpm = commands.len() as f64 / ( self.duration as f64 / 480.0f64 ); // 8 ticks/s x 60s = 480 ticks/min
                        }
                        break;
                    }
                }
            }
        }

        if self.config.clean_file {
            self.file.cleanup();
        }
    }
}
