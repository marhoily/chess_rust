#![allow(collapsible_if)]

use castle;
use castle::Castle;
use bit_board::BitBoard;
use color::Color;
use side::*;
use mask::*;
use mask::masks::*;
use file::File;
use kind::*;
use bit_board::fen;
use self::wrappers::*;
use piece::*;
use moves::Move;

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

    pub fn is_pseudo_legal(&self, mv: Move) -> bool {
        // Source square must not be vacant.
        let from = mv.from.mask();
        let piece = self.board.get_piece(from);
        if piece == VOID {
            return false;
        }
        // Check turn.
        if !self.board.occupation_of(self.active).contains(from) {
            return false;
        }

        //  Only pawns can promote and only on the back-rank.
        if mv.promote != UNKNOWN {
            if self.active == Color::White {
                if piece != WHITE_PAWN {
                    return false;
                }
                if mv.to.rank() != ::rank::_7 {
                    return false;
                }
            } else {
                if piece != BLACK_PAWN {
                    return false;
                }
                if mv.to.rank() != ::rank::_2 {
                    return false;
                }
            }
        }
        if mv.castle != castle::NONE {
            if self.available.contains(mv.castle & self.active.castle()) {
                return true;
            }
        }

        // Destination square can not be occupied.
        let to = mv.to.mask();
        // Do squares occupied by active side contain `to`?
        if self.board.occupation_of(self.active).contains(to) {
            return false;
        }

        // Handle pawn pushes
        if self.board.pawns_of(self.active).contains(to) {
            return self.is_pseudo_legal_pawn_move(from, to);
        }

        // determine which non-pawn piece this is and check it
        true
    }
}

mod validate_position;
mod pawn_moves;

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
