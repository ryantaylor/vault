use crate::data::chunks::Header;
use crate::data::parser::{parse_utf16_variable, parse_utf8_variable};
use crate::data::Item;
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{cond, cut, map};
use nom::multi::length_count;
use nom::number::complete::{le_u32, le_u64, le_u8};
use nom::sequence::tuple;
use nom::IResult;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub human: u8,
    pub name: String,
    pub team: u32,
    pub faction: String,
    _ai_type: String,
    pub steam_id: String,
    pub profile_id: u64,
    _items: Vec<Item>,
}

impl Player {
    pub fn parse_player(header: Header) -> impl FnMut(Span) -> ParserResult<Player> {
        move |input: Span| {
            let (input, player) = cut(map(
                tuple((
                    le_u8,
                    Self::parse_name,
                    Self::parse_team,
                    le_u32,
                    take(1u32),
                    Self::parse_faction,
                    take(8u32),
                    Self::parse_ai,
                    take(40u32),
                    le_u64,
                    take(1u32),
                    Self::parse_steam_id,
                    take(18u32),
                )),
                |(human, name, team, id, _, faction, _, ai_type, _, profile_id, _, steam_id, _)| {
                    Player {
                        id,
                        human,
                        name,
                        team,
                        faction,
                        _ai_type: ai_type,
                        steam_id,
                        profile_id,
                        _items: vec![],
                    }
                },
            ))(input)?;

            let (input, items) = Self::parse_items(input, &player)?;

            // Players in this chunk version or later carry a trailing count-prefixed list.
            // It was empty in every replay up to game version 46673, which is why it used to
            // read as four unexplained bytes between players; 2.5.0 (48652) began populating
            // it — one 6-byte record (u32 pbgid, u16 slot) per entry — and reading only the
            // count then derails the next player's parse.
            let (input, _) = cond(header.version >= 4595383, Self::parse_slots)(input)?;

            Ok((
                input,
                Player {
                    _items: items,
                    ..player
                },
            ))
        }
    }

    #[tracable_parser]
    fn parse_name(input: Span) -> ParserResult<String> {
        let (input, (_, name)) = parse_utf16_variable(le_u32)(input)?;
        Ok((input, name))
    }
    #[tracable_parser]
    fn parse_team(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }
    #[tracable_parser]
    fn parse_faction(input: Span) -> ParserResult<String> {
        let (input, (_, faction)) = parse_utf8_variable(le_u32)(input)?;
        Ok((input, faction))
    }
    #[tracable_parser]
    fn parse_ai(input: Span) -> ParserResult<String> {
        let (input, (_, ai)) = parse_utf8_variable(le_u32)(input)?;
        Ok((input, ai))
    }
    #[tracable_parser]
    fn parse_steam_id(input: Span) -> ParserResult<String> {
        let (input, (_, steam_id)) = parse_utf16_variable(le_u32)(input)?;
        Ok((input, steam_id))
    }

    fn item_parser_for(player: &Player) -> impl FnMut(Span) -> ParserResult<Item> {
        if player.human == 0 {
            Item::parse_cpu_item
        } else {
            Item::parse_player_item
        }
    }

    #[tracable_parser]
    fn parse_items<'a>(input: Span<'a>, player: &Player) -> IResult<Span<'a>, Vec<Item>> {
        cut(map(
            tuple((
                length_count(le_u32, Self::item_parser_for(player)),
                take(4u32),
                length_count(le_u32, Self::item_parser_for(player)),
            )),
            |(mut battlegroup_items, _, mut cosmetic_items)| {
                battlegroup_items.append(&mut cosmetic_items);
                battlegroup_items
            },
        ))(input)
    }

    /// The trailing per-player list `parse_player` describes. Each record is a pbgid and a
    /// slot index; nothing in the public API exposes them yet, so they are read and dropped
    /// to keep the player boundary correct.
    #[tracable_parser]
    fn parse_slots(input: Span) -> ParserResult<()> {
        let (input, _) = length_count(le_u32, take(6u32))(input)?;
        Ok((input, ()))
    }
}
