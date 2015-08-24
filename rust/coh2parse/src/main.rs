// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
//use std::io::SeekFrom;
use std::path::Path;
use std::string::String;

fn main() {
    // Create a path to the desired file
    let path = Path::new("/home/ryan/replays/test.rec");
    //let display = path.display();

    //let mut buff = [0xff, 0xff];
    //let mut buff: Vec<u8> = Vec::new();

    let mut replay = Replay::new(&path);
    replay.parse();

    /*let mut replay = ReplayFile::new(&path);
    replay.skip_u16();
    println!("{}", replay.read_u16());
    println!("{}", replay.read_text(8));*/
    /*let val: u8 = 1;

    println!("{}", replay.test(val));*/

    // Open the path in read-only mode, returns `io::Result<File>`
    /*let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    match file.read_to_end(&mut buff) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}\n{}\n", display, buff[2], buff[3]),
    }

    let slice = &buff[2..3+1];

    println!("{}\n{}", testfn(slice[0]), slice[1]);
    println!("{}", ((slice[0] as u16) << 8));*/

    // Read the file contents into a string, returns `io::Result<usize>`
    //let mut s = String::new();
    /*match file.read(&mut buff) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}\n{}\n", display, buff[0], buff[1]),
    }*/

    /*match file.seek(SeekFrom::Start(0)) {
        Err(why) => panic!("failed to seek: {}", Error::description(&why)),
        Ok(_) => println!("seek successful"),
    }*/

    /*match file.read(&mut buff) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}\n{}\n", display, buff[0], buff[1]),
    }*/

    /*let mut result_u32;
    let mut result_u16: u16 = buff[1];
    result_u16 = result_u16 << 8;

    result_u32 = buff[0] + result_u16;
    println!("{}", result_u32);*/
    // `file` goes out of scope, and the "hello.txt" file gets closed
}

struct ReplayFile {
    file: File,
    data: Vec<u8>,
    cursor: i32,
}

impl ReplayFile {
    fn new(path: &Path) -> ReplayFile {
        let mut replay = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(),
                                                       Error::description(&why)),
            Ok(file) => file,
        };

        let mut buff: Vec<u8> = Vec::new();
        match replay.read_to_end(&mut buff) {
            Err(why) => panic!("couldn't read {}: {}", path.display(),
                                                       Error::description(&why)),
            Ok(_) => println!("{} opened and read into memory", path.display()),
        };

        ReplayFile {
            file: replay,
            data: buff,
            cursor: 0,
        }
    }

    fn skip(&mut self, pos: i32) {
        self.cursor += pos;

        if self.cursor < 0 {
            panic!("ReplayFile::skip - cursor {} is less than minimum value 0", self.cursor);
        }
    }

    fn skip_u8(&mut self) {
        self.skip(1);
    }

    fn skip_u16(&mut self) {
        self.skip(2);
    }

    fn skip_u32(&mut self) {
        self.skip(4);
    }

    fn skip_u64(&mut self) {
        self.skip(8);
    }

    fn seek(&mut self, pos: i32) {
        if pos < 0 {
            panic!("ReplayFile::seek - cursor {} is less than minimum value 0", pos);
        }

        self.cursor = pos;
    }

    fn read_u8(&mut self) -> u8 {
        let result: u8 = self.data[self.cursor as usize];
        self.cursor += 1;
        result
    }

    fn read_u16(&mut self) -> u16 {
        let stream = &self.data[self.cursor as usize..(self.cursor + 2) as usize];
        let result: u16 = ((stream[1] as u16) << 8) + (stream[0] as u16);
        self.cursor += 2;
        result
    }

    fn read_u32(&mut self) -> u32 {
        let stream = &self.data[self.cursor as usize..(self.cursor + 4) as usize];
        let result: u32 = ((stream[3] as u32) << 24) + 
                          ((stream[2] as u32) << 16) +
                          ((stream[1] as u32) << 8) +
                          (stream[0] as u32);
        self.cursor += 4;
        result
    }

    fn read_u64(&mut self) -> u64 {
        let stream = &self.data[self.cursor as usize..(self.cursor + 8) as usize];
        let result: u64 = ((stream[7] as u64) << 56) +
                          ((stream[6] as u64) << 48) +
                          ((stream[5] as u64) << 40) +
                          ((stream[4] as u64) << 32) +
                          ((stream[3] as u64) << 24) + 
                          ((stream[2] as u64) << 16) +
                          ((stream[1] as u64) << 8) +
                          (stream[0] as u64);
        self.cursor += 8;
        result
    }

    fn read_text(&mut self, len: i32) -> String {
        if len < 0 {
            panic!("ReplayFile::read_text - length {} is less than minimum length 0", len);
        }

        let stream = &self.data[self.cursor as usize..(self.cursor + len) as usize];
        let mut stream_vec = Vec::new();
        stream_vec.extend(stream.iter().cloned());

        let result = match String::from_utf8(stream_vec) {
            Err(why) => panic!("ReplayFile::read_text - couldn't read text, cursor {} len {}: {}", self.cursor,
                                                                                                   len,
                                                                                                   Error::description(&why)),
            Ok(text) => text,
        };

        self.cursor += len;
        result
    }
}

struct Replay {
    file: ReplayFile,
    version: u16,
    game_type: String,
    date_time: String,
    map_file: String,
    map_name: String,
    map_description: String,
    map_width: u32,
    map_height: u32,
    players: Vec<Player>,
    duration: u32,
}

impl Replay {
    fn new(path: &Path) -> Replay {
        Replay {
            file: ReplayFile::new(&path),
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
        }
    }

    fn parse(&mut self) {
        self.file.skip_u16();
        self.version = self.file.read_u16();
        self.game_type = self.file.read_text(8);

        let mut ch: String;
        let mut val = self.file.read_u16();

        while val > 0 {
            self.file.skip(-2);
            ch = self.file.read_text(2);
            self.date_time.push_str(&ch);
            val = self.file.read_u16();
        }

        self.file.seek(76);

        self.display();

        self.parse_chunky();
        self.parse_chunky();

        self.parse_data();

        /*ch = self.file.read_text(2);
        let test = String::from(ch);
        println!("{}", &ch);
        if ch == test {
            println!("true!");
        }*/
        /*while !ch.is_empty() {
            self.date_time.push_str(&ch);
            println!("{}", self.date_time);
            ch = self.file.read_text(2);
        }*/
        //self.date_time = ch;

        self.display();
    }

    fn parse_chunky(&mut self) {
        println!("parsing chunky");
        let chunky_type = self.file.read_text(12);
        if chunky_type != "Relic Chunky" {
            panic!("Replay::parse_chunky - Expected chunky type 'Relic Chunky', got '{}'", chunky_type);
        }

        self.file.skip_u32();

        let chunky_version = self.file.read_u32();
        if chunky_version != 3 {
            panic!("Replay::parse_chunky - Expected chunky version 3, got {}", chunky_version);
        }

        self.file.skip_u32();
        let size = self.file.read_u32() as i32;
        self.file.skip(size - 28);

        let mut count = 0;

        while self.parse_chunk() {
            self.display();
            count += 1;
        }

        println!("{} chunks parsed", count);
    }

    fn parse_chunk(&mut self) -> bool {
        println!("parsing chunk");
        let chunk_type = self.file.read_text(8);
        if !chunk_type.starts_with("FOLD") && !chunk_type.starts_with("DATA") {
            self.file.skip(-8);
            return false;
        }

        let chunk_version = self.file.read_u32();
        let chunk_length = self.file.read_u32();
        let chunk_name_length = self.file.read_u32();

        self.file.skip_u64();

        let chunk_name: String;
        if chunk_name_length > 0 {
            chunk_name = self.file.read_text(chunk_name_length as i32);
        }

        let start_position = self.file.cursor;

        if chunk_type.starts_with("FOLD") {
            println!("in FOLD chunk");
            while self.file.cursor < start_position + chunk_length as i32 {
                self.parse_chunk();
            }
        }

        if chunk_type == "DATASDSC" && chunk_version == 0x7e3 {
            println!("in DATASDSC chunk");
            self.file.skip(16);

            let mut size = self.file.read_u32() as i32;
            self.file.skip(8 + 2 * size);

            size = self.file.read_u32() as i32;
            self.map_file = self.file.read_text(size);

            self.file.skip(16);

            size = self.file.read_u32() as i32;
            self.map_name = self.file.read_text(size * 2);

            self.file.skip_u32();

            size = self.file.read_u32() as i32;
            self.map_description = self.file.read_text(size * 2);

            self.file.skip_u32();

            self.map_width = self.file.read_u32();
            self.map_height = self.file.read_u32();
        }

        if chunk_type == "DATADATA" && chunk_version == 0xd {
            println!("in DATADATA chunk");
            self.file.skip(18);

            let num_players = self.file.read_u32();

            let mut player: Player;
            for idx in 0..num_players {
                player = self.parse_player();
                self.players.push(player);
            }

            self.file.skip(90);
        }

        self.file.seek(start_position + chunk_length as i32);

        true
    }

    fn parse_player(&mut self) -> Player {
        let mut player = Player::new();

        self.file.skip_u8();

        let mut size = self.file.read_u32() as i32;
        player.name = self.file.read_text(size * 2);

        player.team = self.file.read_u32();
        player.faction = self.file.read_u32();

        self.file.skip(55);

        player.steam_id = self.file.read_u64();

        self.file.skip_u32();

        for idx in 0..3 {
            player.commanders[idx] = self.file.read_u32();
        }

        self.file.skip_u32();

        for idx in 0..3 {
            player.bulletin_ids[idx] = self.file.read_u32();
        }

        self.file.skip_u32();

        let num_bulletins = self.file.read_u32();

        for idx in 0..3 {
            if idx < num_bulletins {
                size = self.file.read_u32() as i32;
                player.bulletin_names.push(self.file.read_text(size));
            }
            else {
                player.bulletin_names.push(String::new());
            }
        }

        self.file.skip(9);

        player
    }

    fn parse_data(&mut self) {
        println!("parsing data");
        let mut count = 0;
        while self.parse_tick() {
            count += 1;
        };

        println!("{} ticks parsed", count);
    }

    fn parse_tick(&mut self) -> bool {
        println!("parsing tick");
        println!("cursor: {}", self.file.cursor);
        println!("len: {}", self.file.data.len());

        if self.file.cursor as usize >= self.file.data.len() {
            return false;
        }

        self.file.skip_u32();
        let tick_size = self.file.read_u32() as i32;

        if tick_size > 0 {
            self.file.skip(tick_size);
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
    }
}

struct Player {
    name: String,
    steam_id: u64,
    team: u32,
    faction: u32,
    commanders: [u32; 3],
    bulletin_ids: [u32; 3],
    bulletin_names: Vec<String>,
}

impl Player {
    fn new() -> Player {
        Player {
            name: String::new(),
            steam_id: 0,
            team: 0,
            faction: 0,
            commanders: [0; 3],
            bulletin_ids: [0; 3],
            bulletin_names: Vec::new(),
        }
    }
}