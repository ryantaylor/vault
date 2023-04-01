extern crate byteorder;
extern crate nom;
extern crate nom_locate;
extern crate nom_tracable;
extern crate serde;

use nom_locate::LocatedSpan;
use nom_tracable::TracableInfo;
use replay::replay_from_data;

mod data;
mod map;
mod message;
mod player;
mod replay;

pub use crate::map::Map;
pub use crate::message::Message;
pub use crate::player::Player;
pub use crate::replay::Replay;

pub fn parse_replay(data: Vec<u8>) -> Replay {
    let info = TracableInfo::new().parser_width(64).fold("term");
    let input: data::Span = LocatedSpan::new_extra(data.as_slice(), info);
    let (_, replay) = data::Replay::from_span(input).unwrap();
    replay_from_data(&replay)
}
