#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A command format with X, Y, and Z coordinates where the command was issued.

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coordinates {
    tick: u32,
    index: u32,
    x: f32,
    y: f32,
    z: f32,
}

impl Coordinates {
    pub(crate) fn new(tick: u32, index: u32, x: f32, y: f32, z: f32) -> Self {
        Self {
            tick,
            index,
            x,
            y,
            z,
        }
    }

    /// This value is the tick at which the command was found while parsing the replay, which
    /// represents the time in the replay at which it was executed. Because CoH3's engine runs at 8
    /// ticks per second, you can divide this value by 8 to get the number of seconds since the
    /// replay began, which will tell you when this command was executed.
    pub fn tick(&self) -> u32 {
        self.tick
    }
    /// This value is the index of the command relative to the player who issued the command.
    /// Indexes start at 1 and increment on every player-issued command, which means you should be
    /// able to look at the maximum index value of the commands associated with a player to
    /// determine how many commands that player issued in a given game.
    pub fn index(&self) -> u32 {
        self.index
    }
    /// The X coordinate at which the command was issued. Can be mapped to the map's coordinate
    /// system to determine the location on the map where the command was given.
    pub fn x(&self) -> f32 {
        self.x
    }
    /// The Y coordinate at which the command was issued. Can be mapped to the map's coordinate
    /// system to determine the location on the map where the command was given.
    pub fn y(&self) -> f32 {
        self.y
    }
    /// The Z coordinate at which the command was issued. Can be mapped to the map's coordinate
    /// system to determine the location on the map where the command was given.
    pub fn z(&self) -> f32 {
        self.z
    }
}
