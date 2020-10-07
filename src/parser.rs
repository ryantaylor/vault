use std::fmt::Debug;
use std::io::Cursor;
use std::ops::{RangeTo, RangeFrom, Deref};
use std::slice::SliceIndex;
use std::str;
use std::string::String;

use byteorder::{LittleEndian, ReadBytesExt};

// use nom::{le_u16, IResult, Needed, need_more, InputTake, InputLength, AtEof, AsBytes, Slice};
// use nom::types::CompleteByteSlice;
use nom::{IResult};
use nom::bytes::complete::{take, take_till, take_while};
// use nom::error::ParseError;
use nom::combinator::{map_res, verify};
use nom::number::complete::{le_u16, le_u32};
use nom::sequence::{preceded};

fn verify_zero_u16(input: &[u8]) -> IResult<&[u8], u16> {
    verify(le_u16, |n: &u16| *n == 0)(input)
}

// fn verify_zero_u16<'a, O2, E: ParseError<&'a [u8]>, F, G>(
//     first: F,
//     second: G
// ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], u16, E> where
//     F: Fn(&'a [u8]) -> IResult<&'a [u8], u16, E>,
//     G: Fn(&O2) -> bool,
//     O2: ?Sized {
//         verify(le_u16, |n: &u16| *n == 0)
//     }
// fn verify_zero_u16<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], u16, E> {
//     verify(le_u16, |n: &u16| *n == 0)
// }
// named!(zero_u16<u16>, verify!(le_u16, |n: u16| n == 0));

pub fn parse_version(input: &[u8]) -> IResult<&[u8], u16> {
    // let zero = verify(le_u16, |n: &u16| *n == 0);
    preceded(verify_zero_u16, le_u16)(input)
}
// named!(pub match_version<u16>,
//     do_parse!(
//         zero_u16 >>
//         version: le_u16 >>
//         (version)
//     )
// );

pub fn parse_utf8<'a>(len: u8) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &str> {
    map_res(take(len), |s: &[u8]| str::from_utf8(s))
}

pub fn parse_game_type(input: &[u8]) -> IResult<&[u8], &str> {
    parse_utf8(8)(input)
}

pub fn parse_utf16(input: &[u8]) -> IResult<&[u8], String> {
    let (input, u8_slice) = take_till(|c| c == 0)(input)?;

    let mut u16_vec = Vec::with_capacity(u8_slice.len() / 2);
    let mut cursor = Cursor::new(u8_slice);

    cursor.read_u16_into::<LittleEndian>(&mut u16_vec).unwrap();

    let result = String::from_utf16(&u16_vec).unwrap();

    Ok((input, result))
}

pub fn take_zeroes(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|n: u8| n == 0)(input)
}

pub fn verify_le_u32<'a>(expected: u32) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], u32> {
    verify(le_u32, move |n: &u32| *n == expected)
}

// fn parse_utf16(input: &[u8]) -> IResult<&[u8], &str> {
//     map_res(take_till(|c| c == 0), |s: &[u8]| String::from_utf16(s as &[u16])?.as_str)(input)
// }

// pub fn match_utf8<'a>(i: &'a [u8], value: &str) -> IResult<&'a [u8], &'a str> {
//     map_res!(i, tag!(value), str::from_utf8)
// }

// named!(pub match_terminated_utf16<String>,
//     map!(
//         many_till!(le_u16, zero_u16),
//         |(result, _)| String::from_utf16_lossy(&result)
//     )
// );

// pub fn cbs_le_u16(i: CompleteByteSlice) -> IResult<CompleteByteSlice, u16> {
//   if i.len() < 2 {
//     need_more(i, Needed::Size(2))
//   } else {
//     let res = ((i[1] as u16) << 8) + i[0] as u16;
//     Ok((CompleteByteSlice(&i[2..]), res))
//   }
// }

// pub fn orig_le_u16(i: &[u8]) -> IResult<&[u8], u16> {
//   if i.len() < 2 {
//     need_more(i, Needed::Size(2))
//   } else {
//     let res = ((i[1] as u16) << 8) + i[0] as u16;
//     Ok((&i[2..], res))
//   }
// }

// // pub fn b_le_u16<T>(i: T) -> IResult<T, u16> where T: InputLength + Deref + AtEof {
// //     if i.input_len() < 2 {
// //         need_more(i, Needed::Size(2))
// //     } else {
// //         let res = ((i[1] as u16) << 8) + i[0] as u16;
// //         Ok((&i[2..], res))
// //     }
// // }

// pub fn g_le_u8<T>(i: T) -> IResult<T, u8>
// where T: InputLength + AtEof + AsBytes + Slice<RangeTo<usize>> + Slice<RangeFrom<usize>> {
//     if i.input_len() < 1 {
//         need_more(i, Needed::Size(1))
//     } else {
//         let buf = i.slice(..1);
//         let bytes = buf.as_bytes();

//         Ok((i.slice(1..), bytes[0]))
//     }
// }

// pub fn g_le_u16<T>(i: T) -> IResult<T, u16>
// where T: InputLength + AtEof + AsBytes + Slice<RangeTo<usize>> + Slice<RangeFrom<usize>> {
//     if i.input_len() < 2 {
//         need_more(i, Needed::Size(2))
//     } else {
//         let buf = i.slice(..2);
//         let bytes = buf.as_bytes();

//         let res = ((bytes[1] as u16) << 8) + bytes[0] as u16;
//         Ok((i.slice(2..), res))
//     }
// }

// pub fn g_le_u32<T>(i: T) -> IResult<T, u32>
// where T: InputLength + AtEof + AsBytes + Slice<RangeTo<usize>> + Slice<RangeFrom<usize>> {
//     if i.input_len() < 4 {
//         need_more(i, Needed::Size(4))
//     } else {
//         let buf = i.slice(..4);
//         let bytes = buf.as_bytes();

//         let res = ((bytes[3] as u32) << 24) + ((bytes[2] as u32) << 16) + ((bytes[1] as u32) << 8) + bytes[0] as u32;
//         Ok((i.slice(4..), res))
//     }
// }

// pub fn g_le_u64<T>(i: T) -> IResult<T, u64>
// where T: InputLength + AtEof + AsBytes + Slice<RangeTo<usize>> + Slice<RangeFrom<usize>> {
//     if i.input_len() < 8 {
//         need_more(i, Needed::Size(8))
//     } else {
//         let buf = i.slice(..8);
//         let bytes = buf.as_bytes();

//         let res = ((bytes[7] as u64) << 56) + ((bytes[6] as u64) << 48) + ((bytes[5] as u64) << 40) + ((bytes[4] as u64) << 32) +
//             ((bytes[3] as u64) << 24) + ((bytes[2] as u64) << 16) + ((bytes[1] as u64) << 8) + bytes[0] as u64;
//         Ok((i.slice(8..), res))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use byteorder::{LittleEndian, WriteBytesExt};

//     use super::*;

//     #[test]
//     fn test_parse_match_version_ok() {
//         let input = [0, 0, 1, 2];
//         let (_, val) = match_version(&input).unwrap();
//         assert_eq!(val, 513);
//     }

//     #[test]
//     fn test_parse_match_version_err_first() {
//         let input = [1, 0, 1, 2];
//         let error = match match_version(&input) {
//             Err(_) => true,
//             _ => false
//         };
//         assert!(error);
//     }

//     #[test]
//     fn test_parse_match_version_err_second() {
//         let input = [0, 1, 1, 2];
//         let error = match match_version(&input) {
//             Err(_) => true,
//             _ => false
//         };
//         assert!(error);
//     }

//     #[test]
//     fn test_match_utf8_ok() {
//         let input = b"COH2_REC";
//         let (_, val) = match_utf8(input, "COH2_REC").unwrap();
//         assert_eq!(val, "COH2_REC");
//     }

//     #[test]
//     fn test_match_utf8_err() {
//         let input = b"COH_REC";
//         let error = match match_utf8(input, "COH2_REC") {
//             Err(_) => true,
//             _ => false
//         };
//         assert!(error);
//     }

//     #[test]
//     fn test_match_terminated_utf16() {
//         let input = to_utf16_slice("11/7/2015");

//         let (_, val) = match_terminated_utf16(&input).unwrap();
//         assert_eq!(val, "11/7/2015");
//     }

//     fn to_utf16_slice(input: &str) -> Vec<u8> {
//         let mut as_vec: Vec<u16> = input.encode_utf16().collect();
//         as_vec.push(0);
//         let slice_u16: &[u16] = &as_vec;

//         let mut u8s: Vec<u8> = Vec::new();
//         for &n in slice_u16 {
//             let _ = u8s.write_u16::<LittleEndian>(n);
//         }

//         u8s
//     }
// }
