use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{cut, flat_map, map, peek};
use nom::number::complete::le_u32;
use nom::sequence::tuple;

#[derive(Debug)]
pub struct Item {
    pub data: Vec<u8>,
}

impl Item {
    pub fn parse_item(input: Span) -> ParserResult<Item> {
        cut(map(
            tuple((
                take(4u32),
                Self::parse_sublength,
                take(20u32),
                flat_map(le_u32, take),
            )),
            |(_, _, _, data)| Item {
                data: data.to_vec(),
            },
        ))(input)
    }

    pub fn get_item_count(faction: &str, version: u16) -> usize {
        if version < 10000 {
            match faction {
                "british_africa" => 21,
                "americans" => 22,
                _ => 25,
            }
        } else {
            match faction {
                "british_africa" => 21,
                "americans" => 22,
                "germans" => 25,
                _ => 28,
            }
        }
    }

    fn parse_sublength(input: Span) -> ParserResult<Span> {
        let (input, id) = peek(le_u32)(input)?;

        if id == 0 {
            take(12u32)(input)
        } else {
            take(4u32)(input)
        }
    }
}
