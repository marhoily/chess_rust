#![allow(dead_code)]

use castle::Castle;
use bit_board::BitBoard;
use geometry::{File, Color};
use fen;
use castle;
use self::wrappers::*;

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
impl ::std::fmt::Display for Position {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let r = self.en_passant.map_or('-', |x| x.char());
        write!(f, "{} {} {} {}", self.board, self.active, self.available, r)
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
    use nom::Err::Position as P;
    use nom::ErrorKind::Custom as C;
    use super::PositionError::*;
    type R<'a, T, X> = ::nom::IResult<&'a [u8], T, X>;

    pub fn wrapped_parse_bit_board(input: &[u8]) -> R<BitBoard, PositionError> {
        ::fen::parse_bit_board(input).map_err(|err| {
            match err {
                P(C(pe), x) => P(C(Board(pe)), x),
                _ => panic!("wrapped_parse_bit_board"),
            }
        })
    }

    pub fn wrapped_parse_color(input: &[u8]) -> R<Color, PositionError> {
        ::geometry::parse_color(input).map_err(|err| {
            match err {
                P(C(pe), x) => P(C(Active(pe)), x),
                _ => panic!("wrapped_parse_color"),
            }
        })
    }

    pub fn wrapped_parse_castle(input: &[u8]) -> R<Castle, PositionError> {
        ::castle::parse_castle(input).map_err(|err| {
            match err {
                P(C(pe), x) => P(C(Available(pe)), x),
                _ => panic!("wrapped_parse_castle"),
            }
        })
    }

    pub fn wrapped_parse_file(input: &[u8]) -> R<File, PositionError> {
        ::geometry::parse_file(input).map_err(|err| {
            match err {
                P(C(pe), x) => P(C(EnPassant(pe)), x),
                _ => panic!("wrapped_parse_file"),
            }
        })
    }

    named!(ws(&[u8]) -> char, char!(' '));
    pub fn wrapped_ws(input: &[u8]) -> R<char, PositionError> {
        ws(input).map_err(|err| {
            match err {
                P(_, x) => P(C(Whitespace), x),
                _ => panic!("wrapped_ws"),
            }
        })
    }
}

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
        assert_eq!(format!("{}",
                Position::parse("8/8/8/8/8/8/8/8 w KQkq e 0 1")),
                "8/8/8/8/8/8/8/8 w KQkq e");
    }
}
