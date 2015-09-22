//! vault is a fully-featured Company of Heroes 2 replay parser valid for replays created on or
//! after the release of the British forces (version 19545).
//!
//! This library contains representations of all replay, map, and player information, including
//! chat and equipped items. Command parsing is also being actively integrated.

#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate zip;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;
use std::result;
use std::sync::{Arc, Mutex};
use std::thread;

use rustc_serialize::json;
use zip::ZipArchive;

pub use self::error::Error;
pub use self::replay::Replay;

pub mod error;
pub mod replay;
mod chat_line;
mod command;
mod item;
mod map;
mod player;
mod stream;

/// Custom Result wrapper for vault, used to return vault::Error from every result.

pub type Result<T> = result::Result<T, Error>;

/// This type is the main entry point for the vault replay parser and provides the cleanest
/// interface for use by external code.

#[derive(Debug, RustcEncodable)]
pub struct Vault {
    replays: Vec<Replay>
}

impl Vault {

    /// Attempts to parse the given file, returning a Vault type populated with the Replay(s) if
    /// successful.
    ///
    /// Currently .rec and .zip (archives) are supported filetypes. When an archive is provided,
    /// all .rec files in the archive will be parsed. All resulting Replays have their raw byte
    /// data cleaned automatically after parse completes, and cannot be mutated.
    ///
    /// # Panics
    ///
    /// When a call to Replay's parse function panics. This will be improved in future versions to
    /// instead return a Result.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate vault;
    ///
    /// use vault::Vault;
    /// use std::path::Path;
    ///
    /// let path = Path::new("/path/to/file");
    /// let results = Vault::parse(&path).unwrap();
    ///
    /// for replay in results.replays().iter() {
    ///     println!("{}", replay.to_json());
    /// }
    /// ```

    pub fn parse(path: &Path) -> Result<Vault> {
        let meta = try!(fs::metadata(path));

        let replays = if meta.is_dir() {
            try!(Vault::parse_dir(path))
        }
        else if meta.is_file() {
            match path.extension() {
                Some(ext) => match ext.to_string_lossy().deref() {
                    "rec" => try!(Vault::parse_rec(path)),
                    "zip" => try!(Vault::parse_zip(path)),
                    _ => return Err(Error::InvalidFileExtension),
                },
                None => return Err(Error::InvalidFileExtension)
            }
        }
        else {
            return Err(Error::InvalidFileExtension);
        };

        Ok(Vault {
            replays: replays
        })
    }

    /// Parses a .rec file.
    ///
    /// # Panics
    ///
    /// If the call to parse panics. This will be handled with a Result in the future.

    fn parse_rec(path: &Path) -> Result<Vec<Replay>> {
        let mut replay = try!(Replay::new(&path));
        replay.parse().unwrap();

        let mut replays: Vec<Replay> = Vec::with_capacity(1);
        replays.push(replay);

        Ok(replays)
    }

    /// Parses .rec files in a .zip archive.
    ///
    /// # Panics
    ///
    /// If a call to parse panics. This will be handled with a Result in the future.

    fn parse_zip(path: &Path) -> Result<Vec<Replay>> {
        let archive_file = try!(File::open(path));
        let mut archive = try!(ZipArchive::new(archive_file));
        let mut replays: Vec<Replay> = Vec::new();
        let mut handles: Vec<_> = Vec::new();

        let size = archive.len();
        for idx in 0..size {
            //let replay_file = Arc::new(Mutex::new(try!(archive.by_index(idx))));
            let mut replay_file = try!(archive.by_index(idx));
            let name = replay_file.name().to_string();
            let path = Path::new(&name);
            //let path = Path::new("test");
            //let replay_file = Arc::new(Mutex::new(replay_file));
            match path.extension() {
                Some(ext) => match ext.to_string_lossy().deref() {
                    "rec" => {
                        let mut buff: Vec<u8> = Vec::with_capacity(replay_file.size() as usize);
                        try!(replay_file.read_to_end(&mut buff));
                        //let buff = buff.to_owned();
                        //let replay_file = replay_file.clone();
                        let handle = thread::spawn(move || {
                            //let mut buff: Vec<u8> = Vec::with_capacity(replay_file.size() as usize);
                            //try!(replay_file.read_to_end(&mut buff));
                            let mut replay = Replay::from_bytes(buff).unwrap();
                            replay.parse().unwrap();
                            replay
                        });
                        handles.push(handle);
                        /*let mut replay = try!(Replay::from_zipfile(&mut replay_file));
                        replay.parse();
                        let replay = replay;
                        replays.push(replay);*/
                    },
                    _ => info!("skipping {}, not a replay", path.display()),
                },
                None => info!("skipping {}, not a replay", path.display())
            }
        }

        for handle in handles {
            match handle.join() {
                Ok(replay) => replays.push(replay),
                Err(err) => error!("parse failed"),
            }
        }

        Ok(replays)
    }

    /// Parses all .rec and .zip files in the given directory.
    ///
    /// # Panics
    ///
    /// If a call to parse panics. This will be handled with a Result in the future.

    fn parse_dir(path: &Path) -> Result<Vec<Replay>> {
        let dir = try!(fs::read_dir(path));
        let mut replays: Vec<Replay> = Vec::new();
        let mut handles: Vec<_> = Vec::new();

        for item in dir {
            let path = try!(item).path();
            let meta = try!(fs::metadata(&path));
            if meta.is_file() {
                match path.extension() {
                    Some(ext) => match ext.to_string_lossy().deref() {
                        "rec" => {
                            let path = path.to_owned();
                            let handle = thread::spawn(move || {
                                Vault::parse_rec(&path).unwrap()
                            });
                            handles.push(handle);
                        },
                        "zip" => {
                            let path = path.to_owned();
                            let handle = thread::spawn(move || {
                                Vault::parse_zip(&path).unwrap()
                            });
                            handles.push(handle);
                        },
                        _ => info!("skipping {}, not a replay or archive", path.display()),
                    },
                    None => info!("skipping {}, not a replay or archive", path.display())
                }
            }
        }

        for handle in handles {
            match handle.join() {
                Ok(result) => replays.extend(result.into_iter()),
                Err(err) => println!("parse failed"),
            }
        }

        Ok(replays)
    }

    /// Returns a reference to the vector of Replays.

    pub fn replays(&self) -> &Vec<Replay> {
        &self.replays
    }

    /// Serializes Vault as JSON String.
    ///
    /// # Panics
    ///
    /// If rustc_serialize::json::encode fails to encode the Vault.

    pub fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }
}

/// Prints out the current vault version and compatible CoH2 game versions.

pub fn print_version() {
    println!("vault v0.1.4");
    println!(" coh2 19545 - 19654");
}