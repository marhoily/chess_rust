use castle;
use castle::Castle;
use bit_board::BitBoard;
use color::Color;
use side::*;
use mask::*;
use file::File;
use kind::*;
use bit_board::fen;
use self::wrappers::*;
use piece::*;

#[derive(Eq, Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub board: BitBoard,
    pub active: Color,
    pub available: Castle,
    pub en_passant: Option<File>,
}

impl Position {
    pub fn parse(input: &str) -> Self {
        parse_position(input.as_bytes()).unwrap().1
    }
}

mod validate_position;
mod validate_move;
mod pawn_moves;
mod king_moves;

use std::fmt::{Display, Formatter, Result};

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
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

mod wrappers;

// "8/8/8/8/8/8/8/8 w KQkq - 0 1"
named!(pub parse_position<&[u8], Position, PositionError>,
    chain!(
        squares: parse_bit_board ~ ws ~
        side: parse_color ~ ws ~
        castle: parse_castle ~ ws ~
        file: parse_file_or_dash,
        || Position {
                board: squares,
                active: side,
                available: castle,
                en_passant: file
        }));

#[cfg(test)]
mod test {
    use super::*;
    use mask::masks::*;

    #[test]
    fn correct_fen() {
        assert_eq!(format!("{}",
                           Position::parse("8/8/8/8/8/8/8/8 w KQkq e 0 1")),
        "8/8/8/8/8/8/8/8 w KQkq e");
    }

    #[test]
    fn en_passant_is_dash() {
        assert_eq!(format!("{}",
                           Position::parse("8/8/8/8/8/8/8/8 w - - 0 1")),
        "8/8/8/8/8/8/8/8 w - -");
    }

    #[test]
    fn en_passant_file_mask_dash() {
        let p = Position::parse("8/8/8/8/8/8/8/8 w - - 0 1");
        assert_eq!(p.en_passant_take_square_mask::<White>(), EMPTY);
    }

    #[test]
    fn en_passant_file_mask_a() {
        let p = Position::parse("8/8/8/8/8/8/8/8 w - a 0 1");
        assert_eq!(p.en_passant_take_square_mask::<White>(), A6);
    }

    #[test]
    fn en_passant_file_mask_e() {
        let p = Position::parse("8/8/8/8/8/8/8/8 b - e 0 1");
        assert_eq!(p.en_passant_take_square_mask::<Black>(), E3);
    }
}
