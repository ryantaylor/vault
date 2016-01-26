//! A module containing a structure used to store parsing options and configuration.

use std::default::Default;

/// This type contains option and configuration information for a call to one of `vault`'s
/// parsing functions.
///
/// `strict`
///
/// `default: false`
///
/// If `true`, `vault` will fail to parse any file that doesn't have the `.rec` file extension,
/// or `.zip` if `parse_archive` or `parse_any` are called. If `false`, a file with no extension
/// will be treated as a `.rec` file and parsing will be attempted.
///
/// `commands`
///
/// `default: true`
///
/// If `true`, `vault` will parse all commands in the replay and store them as `Command` objects
/// in the `Replay` type's `commands` HashMap. If `false`, `vault` will skip entirely any
/// command parsing. Note that setting this command to `false` will improve parsing speed but
/// make it impossible to determine which commanders were selected by which players.
///
/// `command_bytes`
///
/// `default: false`
///
/// If `true`, `vault` will store the full byte sequence of every command in the `bytes` field
/// of the corresponding `Command` object. This is largely a debugging function used to display
/// the command byte sequence in order to improve parsing logic. Note that this setting does
/// nothing if `commands` is set to `false`.
///
/// `clean_file`
///
/// `default: true`
///
/// If `true`, `vault` will empty the internal `data` vector used to store raw replay bytes in
/// the `Replay` object's `file` `Stream` instance. This is done to prevent vast amounts of
/// junk data from being serialized if we wish to convert a `Replay` to JSON.

#[derive(Debug, Copy, Clone, RustcEncodable)]
pub struct Config {
    pub strict: bool,
    pub commands: bool,
    pub command_bytes: bool,
    pub clean_file: bool,
}

impl Config {

    /// Constructs a new `Config` structure with the given options set.

    pub fn new(strict: bool, commands: bool, command_bytes: bool, clean_file: bool) -> Config {
        Config {
            strict: strict,
            commands: commands,
            command_bytes: command_bytes,
            clean_file: clean_file,
        }
    }
}

impl Default for Config {

    /// Constructs a new `Config` structure with default options set.

    fn default() -> Self {
        Config {
            strict: false,
            commands: true,
            command_bytes: false,
            clean_file: true,
        }
    }
}