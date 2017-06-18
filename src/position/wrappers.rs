use super::root::*;
use castle::Castle;
use bit_board::BitBoard;
use color::Color;
use file::File;
use nom::Err::Position as P;
use nom::ErrorKind::Custom as C;
use super::root::PositionError::*;

type R<'a, T, X> = ::nom::IResult<&'a [u8], T, X>;

pub fn parse_bit_board(input: &[u8]) -> R<BitBoard, PositionError> {
    ::bit_board::fen::parse_bit_board(input).map_err(|err| {
        match err {
            P(C(pe), x) => P(C(Board(pe)), x),
            _ => panic!("parse_bit_board"),
        }
    })
}

pub fn parse_color(input: &[u8]) -> R<Color, PositionError> {
    ::color::parse_color(input).map_err(|err| {
        match err {
            P(C(pe), x) => P(C(Active(pe)), x),
            _ => panic!("parse_color"),
        }
    })
}

pub fn parse_castle(input: &[u8]) -> R<Castle, PositionError> {
    ::castle::parse_castle(input).map_err(|err| {
        match err {
            P(C(pe), x) => P(C(Available(pe)), x),
            _ => panic!("parse_castle"),
        }
    })
}

use ::file::parse_file as blah;
named!(parse_file_or_dash_inner(&[u8]) -> Option<File>,
    alt!(
        value!(None, char!('-')) |
        map!(blah, |x| Some(x))));

pub fn parse_file_or_dash(input: &[u8]) -> R<Option<File>, PositionError> {
    parse_file_or_dash_inner(input).map_err(|err| {
        match err {
            P(C(pe), x) => P(C(EnPassant(pe)), x),
            _ => panic!("parse_file"),
        }
    })
}

named!(ws_inner(&[u8]) -> char, char!(' '));
pub fn ws(input: &[u8]) -> R<char, PositionError> {
    ws_inner(input).map_err(|err| {
        match err {
            P(_, x) => P(C(Whitespace), x),
            _ => panic!("ws"),
        }
    })
}
