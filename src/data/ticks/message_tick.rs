use crate::data::ticks::{Message, Tick};
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{cut, flat_map, map, map_parser, peek};
use nom::multi::many_m_n;
use nom::number::complete::le_u32;
use nom::sequence::tuple;

#[derive(Debug)]
pub struct MessageTick {
    pub tick_type: u32,
    pub messages: Vec<Message>,
    pub position: isize,
}

impl MessageTick {
    pub fn parse_tick(input: Span) -> ParserResult<Tick> {
        map(
            tuple((
                le_u32,
                map_parser(flat_map(le_u32, take), Self::parse_message),
            )),
            |(tick_type, messages)| {
                Tick::Message(MessageTick {
                    tick_type,
                    messages,
                    position: -1,
                })
            },
        )(input)
    }

    fn parse_message(input: Span) -> ParserResult<Vec<Message>> {
        let (_, num_messages) = peek(le_u32)(input)?;

        if num_messages == 0 {
            Self::parse_empty_message(input)
        } else {
            Self::parse_content_message(input, num_messages)
        }
    }

    fn parse_empty_message(input: Span) -> ParserResult<Vec<Message>> {
        cut(map(tuple((le_u32, flat_map(le_u32, take))), |(_, _)| {
            Vec::new()
        }))(input)
    }

    fn parse_content_message(input: Span, num_messages: u32) -> ParserResult<Vec<Message>> {
        cut(map(
            tuple((
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                many_m_n(1, num_messages as usize, Message::parse_message),
            )),
            |(_, _, _, _, _, messages)| messages,
        ))(input)
    }
}
