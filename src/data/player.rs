use crate::data::parser::{parse_utf16_variable, parse_utf8_variable};
use crate::data::Item;
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{cut, map};
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
    pub fn parse_player(input: Span) -> ParserResult<Player> {
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
        Ok((
            input,
            Player {
                _items: items,
                ..player
            },
        ))
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

    #[tracable_parser]
    fn parse_items<'a>(
        input: Span<'a>,
        player: &Player
    ) -> IResult<Span<'a>, Vec<Item>> {
        if player.human == 0 {
            let (input, _) = take(48u32)(input)?;
            return Ok((input, vec![]));
        }
        
        cut(
            map(
                tuple((
                    length_count(le_u32, Item::parse_item),
                    take(4u32),
                    length_count(le_u32, Item::parse_item)
                )),
                |(mut battlegroup_items, _, mut cosmetic_items)| {
                    battlegroup_items.append(&mut cosmetic_items);
                    battlegroup_items
                }
            )
        )(input)
    }
}
