#![allow(dead_code)]

use castle::Castle;
use bit_board::BitBoard;
use geometry::{File, Color};

#[derive(Eq, Debug, Copy, Clone, PartialEq)]
pub struct Position {
    board: BitBoard,
    active: Color,
    available: Castle,
    en_passant: Option<File>,
}
use fen;
use fen::parse_bit_borad;
use geometry::{parse_color, parse_file};
use castle;
use castle::parse_castle;
use nom::IResult;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum PositionError {
    Board(fen::ParsingError),
    Active(u32),
    Available(castle::ParsingError),
    EnPassant(u32),
}
fn wrapped_parse_bit_borad(input: &[u8]) -> IResult<&[u8], BitBoard, PositionError> {
    use nom::Err::Position;
    use nom::ErrorKind::Custom;
    use self::PositionError::Board;
    parse_bit_borad(input).map_err(|err| {
        match err {
            Position(Custom(pe), x) => Position(Custom(Board(pe)), x),
            _ => panic!(),
        }
    })
}
fn wrapped_parse_color(input: &[u8]) -> IResult<&[u8], Color, PositionError> {
    use nom::Err::Position;
    use nom::ErrorKind::Custom;
    use self::PositionError::Active;
    parse_color(input).map_err(|err| {
        match err {
            Position(Custom(pe), x) => Position(Custom(Active(pe)), x),
            _ => panic!(),
        }
    })
}
fn wrapped_parse_castle(input: &[u8]) -> IResult<&[u8], Castle, PositionError> {
    use nom::Err::Position;
    use nom::ErrorKind::Custom;
    use self::PositionError::Available;
    parse_castle(input).map_err(|err| {
        match err {
            Position(Custom(pe), x) => Position(Custom(Available(pe)), x),
            _ => panic!(),
        }
    })
}
fn wrapped_parse_file(input: &[u8]) -> IResult<&[u8], File, PositionError> {
    use nom::Err::Position;
    use nom::ErrorKind::Custom;
    use self::PositionError::EnPassant;
    parse_file(input).map_err(|err| {
        match err {
            Position(Custom(pe), x) => Position(Custom(EnPassant(pe)), x),
            _ => panic!(),
        }
    })
}
// "8/8/8/8/8/8/8/8 w KQkq - 0 1"
named!(pub parse_position<&[u8], Position, PositionError>,
    chain!(
        board: wrapped_parse_bit_borad ~
//    char!(' ') ~
        active: wrapped_parse_color ~
//    char!(' ') ~
        available: wrapped_parse_castle ~
//        char!(' ') ~
        en_passant: wrapped_parse_file,
        || Position {
                board: board,
                active: active,
                available: available,
                en_passant: Some(en_passant)
        }));
