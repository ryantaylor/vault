use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{cut, map};
use nom::multi::length_data;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct Item {
    _data: Vec<u8>,
}

impl Item {
    #[tracable_parser]
    pub fn parse_player_item(input: Span) -> ParserResult<Item> {
        cut(map(
            tuple((take(24u32), length_data(le_u32), take(4u32))),
            |(_, data, _): (Span, Span, Span)| Item {
                _data: data.to_vec(),
            },
        ))(input)
    }

    #[tracable_parser]
    pub fn parse_cpu_item(input: Span) -> ParserResult<Item> {
        cut(map(
            tuple((take(8u32), take(4u32))),
            |(data, _): (Span, Span)| Item {
                _data: data.to_vec(),
            },
        ))(input)
    }
}
