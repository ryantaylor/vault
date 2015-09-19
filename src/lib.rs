//! vault is a fully-featured Company of Heroes 2 replay parser valid for replays created on or
//! after the release of the British forces (version 19545).
//!
//! This library contains representations of all replay, map, and player information, including
//! chat and equipped items. Command parsing is also being actively integrated.

#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate zip;

pub mod replay;
pub mod utils;
mod player;
mod stream;
mod item;
mod command;
mod map;
mod chat_line;

use replay::Replay;

use std::fs::File;
use std::path::Path;
use std::ops::Deref;

use zip::read::ZipArchive;

/// This type contains the range of potential error Results for Vault function calls.

#[derive(Debug)]
pub enum VaultError {
    InvalidFileExtension,
    FileReadFailure,
}

/// This type is the main entry point for the vault replay parser and provides the cleanest
/// interface for use by external code.

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

    pub fn parse(path: &Path) -> Result<Vault, VaultError> {
        match path.extension() {
            Some(ext) => match ext.to_string_lossy().deref() {
                "rec" => Vault::parse_rec(path),
                "zip" => Vault::parse_zip(path),
                _ => Err(VaultError::InvalidFileExtension),
            },
            None => Err(VaultError::InvalidFileExtension)
        }
    }

    /// Parses a .rec file.
    ///
    /// # Panics
    ///
    /// If the call to parse panics. This will be handled with a Result in the future.

    fn parse_rec(path: &Path) -> Result<Vault, VaultError> {
        let mut replay = Replay::new(path);
        let mut replays = Vec::with_capacity(1);

        replay.parse();
        replays.push(replay);

        let result = Vault {
            replays: replays
        };

        Ok(result)
    }

    /// Parses .rec files in a .zip archive.
    ///
    /// # Panics
    ///
    /// If a call to parse panics. This will be handled with a Result in the future.

    fn parse_zip(path: &Path) -> Result<Vault, VaultError> {
        let archive_file = match File::open(path) {
            Ok(val) => val,
            Err(_) => return Err(VaultError::FileReadFailure),
        };

        let mut archive = ZipArchive::new(archive_file).unwrap();
        let size = archive.len();

        let mut replays = Vec::with_capacity(size);

        for idx in 0..size {
            let mut replay_file = archive.by_index(idx).unwrap();
            let mut replay = Replay::from_zipfile(&mut replay_file);
            replay.parse();
            replays.push(replay);
        }

        let result = Vault {
            replays: replays
        };

        Ok(result)
    }

    /// Returns a reference to the vector of Replays.

    pub fn replays(&self) -> &Vec<Replay> {
        &self.replays
    }
}