use data::parser::{parse_utf16_variable, parse_utf8_variable};
use data::Item;
use data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::{cut, map};
use nom::multi::many_m_n;
use nom::number::complete::{le_u32, le_u64};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub team: u32,
    pub faction: String,
    pub ai_type: String,
    pub steam_id: String,
    pub profile_id: u64,
    pub items: Vec<Item>,
}

impl Player {
    pub fn parse_player(version: u16) -> impl FnMut(Span) -> ParserResult<Player> {
        move |input: Span| {
            let (input, player) = cut(map(
                tuple((
                    take(1u32),
                    Self::parse_name,
                    Self::parse_team,
                    take(5u32),
                    Self::parse_faction,
                    take(8u32),
                    Self::parse_ai,
                    take(40u32),
                    le_u64,
                    take(1u32),
                    Self::parse_steam_id,
                    take(18u32),
                )),
                |(_, name, team, _, faction, _, ai_type, _, profile_id, _, steam_id, _)| Player {
                    name,
                    team,
                    faction,
                    ai_type,
                    steam_id,
                    profile_id,
                    items: vec![],
                },
            ))(input)?;

            let (input, items) = Self::parse_items(input, &player.faction, version)?;
            let (input, _) = take(4u32)(input)?;
            Ok((input, Player { items, ..player }))
        }
    }

    fn parse_name(input: Span) -> ParserResult<String> {
        let (input, (_, name)) = parse_utf16_variable(le_u32)(input)?;
        Ok((input, name))
    }
    fn parse_team(input: Span) -> ParserResult<u32> {
        le_u32(input)
    }
    fn parse_faction(input: Span) -> ParserResult<String> {
        let (input, (_, faction)) = parse_utf8_variable(le_u32)(input)?;
        Ok((input, faction))
    }
    fn parse_ai(input: Span) -> ParserResult<String> {
        let (input, (_, ai)) = parse_utf8_variable(le_u32)(input)?;
        Ok((input, ai))
    }
    fn parse_steam_id(input: Span) -> ParserResult<String> {
        let (input, (_, steam_id)) = parse_utf16_variable(le_u32)(input)?;
        Ok((input, steam_id))
    }

    fn parse_items<'a>(
        input: Span<'a>,
        faction: &str,
        version: u16,
    ) -> IResult<Span<'a>, Vec<Item>> {
        cut(many_m_n(
            0,
            Item::get_item_count(faction, version),
            Item::parse_item,
        ))(input)
    }
}
