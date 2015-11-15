//! A module containing a structure used to store parsing options and configuration.

use std::default::Default;

/// This type contains option and configuration information for the associated `Vault` instance.

#[derive(Debug, Copy, Clone, RustcEncodable)]
pub struct Config {
    pub strict: bool,
    pub commands: bool,
    pub command_bytes: bool,
}

impl Config {

    /// Constructs a new `Config` structure with the given options set.

    pub fn new(strict: bool, commands: bool, command_bytes: bool) -> Config {
        Config {
            strict: strict,
            commands: commands,
            command_bytes: command_bytes,
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
        }
    }
}