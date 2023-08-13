use nom::IResult;
use nom_locate::LocatedSpan;
use nom_tracable::TracableInfo;

pub mod chunks;
mod chunky;
pub mod commands;
mod header;
mod item;
mod parser;
mod player;
mod replay;
pub mod ticks;

use crate::data::chunky::Chunky;
use crate::data::header::Header;
use crate::data::item::Item;
pub use crate::data::player::Player;
pub use crate::data::replay::Replay;

pub type Span<'a> = LocatedSpan<&'a [u8], TracableInfo>;

pub type ParserResult<'a, T> = IResult<Span<'a>, T>;
