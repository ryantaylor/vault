//! `vault` is a fully-featured Company of Heroes 2 replay parser valid for replays created on or
//! after the release of the British forces (version 19545).
//!
//! This library contains representations of all replay, map, and player information, including
//! chat, equipped items, and player commands.

#![cfg_attr(feature = "dev", feature(test))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

// #![feature(test)]
// extern crate test;
#[cfg(feature = "ffi")]
extern crate libc;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
#[cfg(feature = "parse-archive")]
extern crate zip;
#[macro_use]
extern crate nom;
extern crate byteorder;

#[cfg(feature = "ffi")]
use std::ffi::{CStr, CString};
#[cfg(feature = "parse-all")]
use std::fs;
#[cfg(feature = "parse-archive")]
use std::fs::File;
#[cfg(feature = "parse-archive")]
use std::io::Read;
use std::ops::Deref;
use std::path::Path;
use std::result;
#[cfg(feature = "parse-archive")]
use std::thread;

#[cfg(feature = "ffi")]
use libc::c_char;
#[cfg(feature = "ffi")]
use rustc_serialize::json;
#[cfg(feature = "parse-archive")]
use zip::ZipArchive;

pub use self::chat_line::ChatLine;
pub use self::command::{CmdType, Command};
pub use self::config::Config;
pub use self::error::Error;
pub use self::item::Item;
pub use self::map::Map;
pub use self::player::Player;
pub use self::replay::Replay;

mod replay_service;
mod new_replay;
pub mod parser;

mod chat_line;
mod command;
mod config;
mod error;
mod item;
mod map;
mod player;
mod replay;
mod stream;
#[cfg(test)]
mod tests;

/// Custom `Result` wrapper for `vault`, used to return `vault::Error` from every result.

pub type Result<T> = result::Result<T, Error>;

/// Parses a single replay (`.rec`) file.
///
/// If `None` is passed to `config`, the following default configuration is used:
///
/// ```text
/// strict = false
/// commands = true
/// command_bytes = false
/// clean_file = true
/// ```
///
/// # Examples
///
/// ```ignore
/// extern crate vault;
///
/// use std::path::Path;
///
/// let path = Path::new("/path/to/file");
/// let replay = vault::parse_replay(&path, None).unwrap();
/// ```

pub fn parse_replay(path: &Path, config: Option<Config>) -> Result<Replay> {
    let cfg = match config {
        Some(val) => val,
        None => Config::default()
    };

    if cfg.strict {
        match path.extension() {
            Some(ext) => match ext.to_string_lossy().deref() {
                "rec" => {
                    let mut replay = try!(Replay::new(path, cfg));
                    replay.parse();
                    Ok(replay)
                },
                _ => Err(Error::InvalidFileExtension),
            },
            None => Err(Error::InvalidFileExtension),
        }
    } else {
        let mut replay = try!(Replay::new(path, cfg));
        replay.parse();
        Ok(replay)
    }
}

/// Parses all replay (`.rec`) files in a `.zip` archive. This function can be enabled with the
/// `parse-archive` feature.
///
/// If `None` is passed to `config`, the following default configuration is used:
///
/// ```text
/// strict = false
/// commands = true
/// command_bytes = false
/// clean_file = true
/// ```
///
/// # Note
///
/// The `strict` `Config` parameter is not checked in this function; only files with the `.rec`
/// extension in the archive will be parsed, and only archives with the `.zip` extension will be
/// accepted.
///
/// # Examples
///
/// ```ignore
/// extern crate vault;
///
/// use std::path::Path;
///
/// let path = Path::new("/path/to/archive.zip");
/// let replays = vault::parse_archive(&path, None).unwrap();
/// ```

#[cfg(feature = "parse-archive")]
pub fn parse_archive(path: &Path, config: Option<Config>) -> Result<Vec<Replay>> {
    let archive_file = try!(File::open(path));
    let mut archive = try!(ZipArchive::new(archive_file));
    let mut handles: Vec<_> = Vec::new();
    let mut replays: Vec<Replay> = Vec::new();
    let cfg = match config {
        Some(val) => val,
        None => Config::default()
    };

    let size = archive.len();
    for idx in 0..size {
        let mut replay_file = match archive.by_index(idx) {
            Ok(file) => file,
            Err(_) => {
                error!("cannot read file at index {}", idx);
                continue;
            }
        };
        let name = replay_file.name().to_owned();
        let inner_path = Path::new(&name);
        match inner_path.extension() {
            Some(ext) => match ext.to_string_lossy().deref() {
                "rec" => {
                    let base_name = path.to_string_lossy();
                    let base_name = base_name.deref();
                    let mut combo_name = String::from(base_name);
                    combo_name.push_str(":");
                    combo_name.push_str(&name);

                    let mut buff: Vec<u8> = Vec::with_capacity(replay_file.size() as usize);
                    match replay_file.read_to_end(&mut buff) {
                        Ok(_) => (),
                        Err(err) => {
                            replays.push(Replay::new_with_error(&combo_name, Error::IoError(err)));
                            continue;
                        }
                    }

                    let config = cfg.clone();
                    let handle = thread::spawn(move || {
                        match Replay::from_bytes(&combo_name, buff, config) {
                            Ok(replay) => {
                                let mut replay = replay;
                                replay.parse();
                                replay
                            },
                            Err(err) => Replay::new_with_error(&combo_name, err),
                        }
                    });
                    handles.push(handle);
                },
                _ => info!("skipping {}, not a replay", path.display()),
            },
            None => info!("skipping {}, not a replay", path.display())
        }
    }

    for handle in handles {
        match handle.join() {
            Ok(replay) => replays.push(replay),
            Err(_) => error!("parse failed"),
        }
    }
    let replays = replays;

    Ok(replays)
}

/// Parses the given filepath based on its metadata. Accepted are directories, replay files (with or
/// without `.rec` based on the value of `strict`), and archives (`.zip`). The return value is always
/// a `Vec`, even if a single replay file is given as input. This function can be enabled with the
/// `parse-all` feature.
///
/// If `None` is passed to `config`, the following default configuration is used:
///
/// ```text
/// strict = false
/// commands = true
/// command_bytes = false
/// clean_file = true
/// ```
///
/// # Examples
///
/// ```ignore
/// extern crate vault;
///
/// use std::path::Path;
///
/// let path = Path::new("/path/to/replay.rec");
/// let replays = vault::parse_any(&path, None).unwrap();
///
/// let path = Path::new("/path/to/archive.zip");
/// let replays = vault::parse_any(&path, None).unwrap();
///
/// let path = Path::new("/path/to/directory");
/// let replays = vault::parse_any(&path, None).unwrap();
/// ```

#[cfg(feature = "parse-all")]
pub fn parse_any(path: &Path, config: Option<Config>) -> Result<Vec<Replay>> {
    let meta = try!(fs::metadata(path));
    let cfg = match config {
        Some(val) => val,
        None => Config::default()
    };

    if meta.is_dir() {
        Ok(try!(parse_directory(path, config)))
    }
    else if meta.is_file() {
        match path.extension() {
            Some(ext) => match ext.to_string_lossy().deref() {
                "rec" => Ok(try!(parse_rec(path, cfg))),
                "zip" => Ok(try!(parse_archive(path, config))),
                _ => Err(Error::InvalidFileExtension),
            },
            None => {
                if cfg.strict {
                    Err(Error::InvalidFileExtension)
                }
                else {
                    Ok(try!(parse_rec(path, cfg)))
                }
            }
        }
    }
    else {
        Err(Error::InvalidFileExtension)
    }
}

/// Helper method to parse a `.rec` file in a directory and return it as a `Vec`. This
/// function can be enabled with the `parse-all` feature.

#[cfg(feature = "parse-all")]
fn parse_rec(path: &Path, config: Config) -> Result<Vec<Replay>> {
    let mut replay = try!(Replay::new(&path, config));
    replay.parse();
    let replay = replay;

    let mut replays: Vec<Replay> = Vec::with_capacity(1);
    replays.push(replay);
    let replays = replays;

    Ok(replays)
}

/// Parses all replay and archive (`.zip`) files in the first level of the given directory.
/// This function can be enabled with the `parse-all` feature.
///
/// If `None` is passed to `config`, the following default configuration is used:
///
/// ```text
/// strict = false
/// commands = true
/// command_bytes = false
/// clean_file = true
/// ```
///
/// # Examples
///
/// ```ignore
/// extern crate vault;
///
/// use std::path::Path;
///
/// let path = Path::new("/path/to/directory");
/// let replays = vault::parse_directory(&path, None).unwrap();
/// ```

#[cfg(feature = "parse-all")]
pub fn parse_directory(path: &Path, config: Option<Config>) -> Result<Vec<Replay>> {
    let dir = try!(fs::read_dir(path));
    let mut replays: Vec<Replay> = Vec::new();
    let mut handles: Vec<_> = Vec::new();
    let cfg = match config {
        Some(val) => val,
        None => Config::default()
    };

    for item in dir {
        let item = match item {
            Ok(val) => val,
            Err(_) => {
                error!("error reading file in directory");
                continue;
            }
        };

        let path = item.path();
        let meta = match fs::metadata(&path) {
            Ok(val) => val,
            Err(_) => {
                error!("error reading file in directory");
                continue;
            }
        };

        if meta.is_file() {
            match path.extension() {
                Some(ext) => match ext.to_string_lossy().deref() {
                    "rec" => {
                        let path = path.to_owned();
                        let config = cfg.clone();
                        let handle = thread::spawn(move || {
                            match parse_rec(&path, config) {
                                Ok(results) => results,
                                Err(err) => {
                                    let mut result = Vec::with_capacity(1);
                                    let long_name = path.to_string_lossy();
                                    let long_name = long_name.deref();
                                    result.push(Replay::new_with_error(long_name, err));
                                    result
                                }
                            }
                        });
                        handles.push(handle);
                    },
                    "zip" => {
                        let path = path.to_owned();
                        let config = cfg.clone();
                        let handle = thread::spawn(move || {
                            match parse_archive(&path, Some(config)) {
                                Ok(results) => results,
                                Err(err) => {
                                    let mut result = Vec::with_capacity(1);
                                    let long_name = path.to_string_lossy();
                                    let long_name = long_name.deref();
                                    result.push(Replay::new_with_error(long_name, err));
                                    result
                                }
                            }
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
            Err(_) => error!("parse failed"),
        }
    }

    Ok(replays)
}

/// Prints out the current vault version and compatible CoH2 game versions.

pub fn print_version() {
    println!("vault v{}", env!("CARGO_PKG_VERSION"));
}

/// Extern function for invoking a parse operation across FFI. Returns a Vault type serialized to
/// JSON.
///
/// Note that the return type is a pointer to a `c_char` array. The function passes ownership of
/// the `CString` to the FFI caller and does not deallocate memory when the `CString` goes out of
/// this function's scope. It is the responsibility of the FFI caller to pass back the `*c_char` to
/// this library via `free_cstring` so that it can be deallocated. Failure to pass back to
/// `free_cstring` will result in a memory leak.
///
/// # Examples
///
/// ```javascript
/// // node.js
///
/// var ffi = require('ffi');
/// var ref = require('ref');
///
/// var charPtr = ref.refType(ref.types.CString);
///
/// var lib = ffi.Library('/path/to/vault/target/release/libvault', {
///     'parse_to_cstring': [charPtr, ['string']],
///     'free_cstring': ['void', [charPtr]]
/// });
///
/// var path = '/path/to/rec/zip/or/dir';
/// var ptr = lib.parse_to_cstring(path);
/// var str = ref.readCString(ptr, 0);
/// lib.free_cstring(ptr);
///
/// console.log(str);
/// ```

#[cfg(feature = "ffi")]
#[no_mangle]
pub extern fn parse_to_cstring(path: *const c_char) -> *mut c_char {
    let cstr = unsafe { CStr::from_ptr(path) };
    let cow = cstr.to_string_lossy();
    let path_str = cow.deref();
    let path = Path::new(&path_str);
    let replay = parse_replay(&path, None).unwrap();
    let result = json::encode(&replay).unwrap();
    let val = CString::new(result.into_bytes()).unwrap();
    val.into_raw()
}

/// Extern function for deallocating a `CString` returned by `parse_to_cstring`.
///
/// Must only be passed a pointer created by `parse_to_cstring`; passing other pointers is
/// undefined behaviour, and will likely cause a seg fault or double free. Every call to
/// `parse_to_cstring` should have a matching `free_cstring` to deallocate the memory after the
/// string has been used. Failure to call this function will result in a memory leak.

#[cfg(feature = "ffi")]
#[no_mangle]
pub extern fn free_cstring(ptr: *mut c_char) {
    let _ = unsafe { CString::from_raw(ptr) };
}
