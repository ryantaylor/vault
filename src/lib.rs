//! vault is a fully-featured Company of Heroes 2 replay parser valid for replays created on or
//! after the release of the British forces (version 19545).
//!
//! This library contains representations of all replay, map, and player information, including
//! chat and equipped items. Command parsing is also being actively integrated.

#[macro_use]
extern crate log;
extern crate rustc_serialize;

pub mod replay;
pub mod utils;
mod player;
mod stream;
mod item;
mod command;
mod map;
mod chat_line;