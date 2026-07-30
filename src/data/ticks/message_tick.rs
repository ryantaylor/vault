use crate::data::ticks::{Message, Tick};
use crate::data::{ParserResult, Span};
use nom::combinator::{cut, map};
use nom::multi::{length_data, length_value};
use nom::number::complete::le_u32;
use nom::sequence::tuple;

#[derive(Debug)]
pub struct MessageTick {
    _tick_type: u32,
    pub messages: Vec<Message>,
}

impl MessageTick {
    pub fn parse_tick(input: Span) -> ParserResult<Tick> {
        map(
            tuple((le_u32, length_value(le_u32, Self::parse_message))),
            |(tick_type, messages)| {
                Tick::Message(MessageTick {
                    _tick_type: tick_type,
                    messages,
                })
            },
        )(input)
    }

    /// The payload opens with a kind, followed by the length of everything after it. Kind 1 is
    /// chat and kind 0 carries nothing; both have been present since well before this parser.
    /// Game version 48652 (patch 2.5.0) added kind 2, whose body is four bytes — shorter than
    /// the chat header. Reading that leading value as a message count, as this parser used to,
    /// then runs off the end of the payload and fails the entire replay, so anything that is
    /// not chat is now skipped by its own declared length instead.
    fn parse_message(input: Span) -> ParserResult<Vec<Message>> {
        let (input, kind) = le_u32(input)?;

        if kind == 1 {
            Self::parse_chat_message(input)
        } else {
            Self::parse_no_message(input)
        }
    }

    fn parse_no_message(input: Span) -> ParserResult<Vec<Message>> {
        cut(map(length_data(le_u32), |_| Vec::new()))(input)
    }

    /// A slot, an eight-byte sender id, then the message. Parsed inside the body's own length
    /// so that a body carrying more than we know how to read cannot desynchronise the ticks.
    fn parse_chat_message(input: Span) -> ParserResult<Vec<Message>> {
        cut(length_value(
            le_u32,
            map(
                tuple((le_u32, le_u32, le_u32, Message::parse_message)),
                |(_, _, _, message)| vec![message],
            ),
        ))(input)
    }
}
