//! vault is a fully-featured Company of Heroes 2 replay parser valid for replays created on or
//! after the release of the British forces (version 19545).
//!
//! This library contains representations of all replay, map, and player information, including
//! chat and equipped items. Command parsing is also being actively integrated.

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[cfg(feature = "ffi")]
extern crate libc;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate zip;

#[cfg(feature = "ffi")]
use std::ffi::{CStr, CString};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;
use std::result;
use std::thread;

#[cfg(feature = "ffi")]
use libc::c_char;
use rustc_serialize::json;
use zip::ZipArchive;

pub use self::command::Command;
pub use self::error::Error;
//pub use self::player::Player;
pub use self::replay::Replay;

mod chat_line;
mod command;
mod error;
mod item;
mod map;
mod player;
mod replay;
mod stream;

/// Custom Result wrapper for vault, used to return vault::Error from every result.

pub type Result<T> = result::Result<T, Error>;

/// This type is the main entry point for the vault replay parser and provides the cleanest
/// interface for use by external code.

#[derive(Debug, RustcEncodable)]
pub struct Vault {
    pub replays: Vec<Replay>
}

impl Vault {

    /// Attempts to parse the given file, returning a Vault type populated with the Replay(s) if
    /// successful.
    ///
    /// Currently .rec and .zip (archives) are supported filetypes. When an archive is provided,
    /// all .rec files in the archive will be parsed. All resulting Replays have their raw byte
    /// data cleaned automatically after parse completes, and cannot be mutated.
    ///
    /// # Examples
    ///
    /// ```ignore
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
                None => return Err(Error::InvalidFileExtension),
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

    fn parse_rec(path: &Path) -> Result<Vec<Replay>> {
        let mut replay = try!(Replay::new(&path));
        replay.parse();
        let replay = replay;

        let mut replays: Vec<Replay> = Vec::with_capacity(1);
        replays.push(replay);
        let replays = replays;

        Ok(replays)
    }

    /// Parses .rec files in a .zip archive.

    fn parse_zip(path: &Path) -> Result<Vec<Replay>> {
        let archive_file = try!(File::open(path));
        let mut archive = try!(ZipArchive::new(archive_file));
        let mut handles: Vec<_> = Vec::new();
        let mut replays: Vec<Replay> = Vec::new();

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

                        let handle = thread::spawn(move || {
                            match Replay::from_bytes(&combo_name, buff) {
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

    /// Parses all .rec and .zip files in the given directory.

    fn parse_dir(path: &Path) -> Result<Vec<Replay>> {
        let dir = try!(fs::read_dir(path));
        let mut replays: Vec<Replay> = Vec::new();
        let mut handles: Vec<_> = Vec::new();

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
                            let handle = thread::spawn(move || {
                                match Vault::parse_rec(&path) {
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
                            let handle = thread::spawn(move || {
                                match Vault::parse_zip(&path) {
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

    /// Serializes Vault as JSON String.

    pub fn to_json(&self) -> Result<String> {
        Ok(try!(json::encode(&self)))
    }
}

/// Prints out the current vault version and compatible CoH2 game versions.

pub fn print_version() {
    println!("vault v{}", env!("CARGO_PKG_VERSION"));
}

/// Extern function for invoking a parse operation across FFI. Returns a Vault type serialized to
/// JSON.
///
/// Note that the return type is a pointer to a c_char array. The function passes ownership of the
/// CString to the FFI caller and does not deallocate memory when the CString goes out of this
/// function's scope. It is the responsibility of the FFI caller to pass back the *c_char to this
/// library via free_cstring so that it can be deallocated. Failure to pass back to free_cstring
/// will result in a memory leak.
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
    let vault = Vault::parse(&path).unwrap();
    let result = vault.to_json().unwrap();
    let val = CString::new(result.into_bytes()).unwrap();
    val.into_raw()
}

/// Extern function for deallocating a CString returned by parse_to_cstring.
///
/// Must only be passed a pointer created by parse_to_cstring; passing other pointers is undefined
/// behaviour, and will likely cause a seg fault or double free. Every call to parse_to_cstring
/// should have a matching free_cstring to deallocate the memory after the string has been used.
/// Failure to call this function will result in a memory leak.

#[cfg(feature = "ffi")]
#[no_mangle]
pub extern fn free_cstring(ptr: *mut c_char) {
    let _ = unsafe { CString::from_raw(ptr) };
}