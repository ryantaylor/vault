use crate::data::Span;
use byteorder::{LittleEndian, ReadBytesExt};
use nom::bytes::complete::{take, take_while};
use nom::combinator::{map, peek, verify};
use nom::error::ParseError;
use nom::multi::many_till;
use nom::number::complete::{le_u16, le_u32};
use nom::{IResult, ToUsize};
use std::io::Cursor;
use std::string::String;

pub fn verify_zero_u16(input: Span) -> IResult<Span, u16> {
    verify(le_u16, |n: &u16| *n == 0)(input)
}

pub fn verify_le_u32<'a>(expected: u32) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, u32> {
    verify(le_u32, move |n: &u32| *n == expected)
}

pub fn parse_utf8_fixed<'a, E, T: ToUsize>(
    len: T,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>>,
{
    map(take(len), |s: Span| {
        String::from_utf8_lossy(s.fragment()).into_owned()
    })
}

pub fn parse_utf8_variable<'a, O, E, F>(
    mut f: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, (O, String), E>
where
    E: ParseError<Span<'a>>,
    F: FnMut(Span<'a>) -> IResult<Span<'a>, O, E>,
    O: ToUsize + Copy,
{
    move |input: Span| {
        let (input, num) = f(input)?;
        let (input, res) = parse_utf8_fixed(num)(input)?;

        Ok((input, (num, res)))
    }
}

fn bytes_to_utf16(bytes: Span) -> String {
    let mut u16_vec = Vec::with_capacity(bytes.len() / 2);
    let mut cursor = Cursor::new(bytes.fragment());

    for _ in 1..=(bytes.len() / 2) {
        let val = cursor.read_u16::<LittleEndian>().unwrap();
        u16_vec.push(val);
    }

    String::from_utf16_lossy(&u16_vec)
}

pub fn parse_utf16_terminated(input: Span) -> IResult<Span, String> {
    map(
        many_till(le_u16, peek(verify(le_u16, |n: &u16| *n == 0))),
        |(u16s, _)| String::from_utf16_lossy(&u16s),
    )(input)
}

pub fn parse_utf16_fixed<'a, E, T>(len: T) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>>,
    T: ToUsize,
{
    let len = len.to_usize();
    let true_len = len * 2;

    map(take(true_len), bytes_to_utf16)
}

pub fn parse_utf16_variable<'a, O, E, F>(
    mut f: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, (O, String), E>
where
    E: ParseError<Span<'a>>,
    F: FnMut(Span<'a>) -> IResult<Span<'a>, O, E>,
    O: ToUsize + Copy,
{
    move |input: Span| {
        let (input, num) = f(input)?;
        let (input, res) = parse_utf16_fixed(num)(input)?;

        Ok((input, (num, res)))
    }
}

pub fn take_zeroes(input: Span) -> IResult<Span, Span> {
    take_while(|n: u8| n == 0)(input)
}
