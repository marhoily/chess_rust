#![allow(dead_code)]

use castle::Castle;
use bit_board::BitBoard;
use geometry::{File, Color};
use fen;
use castle;

#[derive(Eq, Debug, Copy, Clone, PartialEq)]
pub struct Position {
    board: BitBoard,
    active: Color,
    available: Castle,
    en_passant: Option<File>,
}
impl Position {
    pub fn parse(input: &str) -> Self {
        parse_position(input.as_bytes()).unwrap().1
    }
}

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum PositionError {
    Board(fen::ParsingError),
    Active(u32),
    Available(castle::ParsingError),
    EnPassant(u32),
    Whitespace,
}
mod wrappers {
    use super::*;
    use castle::Castle;
    use bit_board::BitBoard;
    use geometry::{File, Color};
    use fen::parse_bit_board;
    use geometry::{parse_color, parse_file};
    use castle::parse_castle;
    use nom::IResult;
    use nom::Err::Position as Positional;
    use nom::ErrorKind::Custom;
    use super::PositionError::*;

    pub fn wrapped_parse_bit_board(input: &[u8]) -> IResult<&[u8], BitBoard, PositionError> {
        parse_bit_board(input).map_err(|err| {
            match err {
                Positional(Custom(pe), x) => Positional(Custom(Board(pe)), x),
                _ => panic!("wrapped_parse_bit_board"),
            }
        })
    }

    pub fn wrapped_parse_color(input: &[u8]) -> IResult<&[u8], Color, PositionError> {
        parse_color(input).map_err(|err| {
            match err {
                Positional(Custom(pe), x) => Positional(Custom(Active(pe)), x),
                _ => panic!("wrapped_parse_color"),
            }
        })
    }

    pub fn wrapped_parse_castle(input: &[u8]) -> IResult<&[u8], Castle, PositionError> {
        parse_castle(input).map_err(|err| {
            match err {
                Positional(Custom(pe), x) => Positional(Custom(Available(pe)), x),
                _ => panic!("wrapped_parse_castle"),
            }
        })
    }

    pub fn wrapped_parse_file(input: &[u8]) -> IResult<&[u8], File, PositionError> {
        parse_file(input).map_err(|err| {
            match err {
                Positional(Custom(pe), x) => Positional(Custom(EnPassant(pe)), x),
                _ => panic!("wrapped_parse_file"),
            }
        })
    }

    named!(ws(&[u8]) -> char, char!(' '));
    pub fn wrapped_ws(input: &[u8]) -> IResult<&[u8], char, PositionError> {
        ws(input).map_err(|err| {
            match err {
                Positional(_, x) => Positional(Custom(Whitespace), x),
                _ => panic!("wrapped_ws"),
            }
        })
    }
}
use self::wrappers::*;

// "8/8/8/8/8/8/8/8 w KQkq - 0 1"
named!(pub parse_position<&[u8], Position, PositionError>,
    chain!(
        board: wrapped_parse_bit_board ~
        wrapped_ws ~
        active: wrapped_parse_color ~
        wrapped_ws ~
        available: wrapped_parse_castle ~
        wrapped_ws ~
        en_passant: wrapped_parse_file,
        || Position {
                board: board,
                active: active,
                available: available,
                en_passant: Some(en_passant)
        }));
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_fen() {
        assert_eq!(format!("{:?}",
                parse_position(b"8/8/8/8/8/8/8/8 w KQkq e 0 1")),
                "Done([32, 48, 32, 49], \
                    Position { \
                        board: BitBoard(\
                            [Mask(0), Mask(0), Mask(0), Mask(0), Mask(0), Mask(0), Mask(0), \
                             Mask(0), Mask(0), Mask(0), Mask(0), Mask(0)]), \
                        active: White, \
                        available: Q | K | W | B | WQ | WK | BQ | BK | ALL, \
                        en_passant: Some(File(4)) \
                })");
    }
}
