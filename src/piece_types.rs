use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use colored_squares::*;
use super::pieces::*;

pub const PIECE_TYPES_COUNT: u8 = 6;

static PIECE_CHARS: &'static [u8; 12] = b"PNBRQKpnbrqk";

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct PieceType(u8);

pub const PAWN: PieceType = PieceType(0);
pub const KNIGHT: PieceType = PieceType(1);
pub const BISHOP: PieceType = PieceType(2);
pub const ROOK: PieceType = PieceType(3);
pub const QUEEN: PieceType = PieceType(4);
pub const KING: PieceType = PieceType(5);
pub const UNKNOWN: PieceType = PieceType(16);

impl PieceType {
    pub fn new(bits: u8) -> Self {
        PieceType(bits)
    }
    pub fn of(self, color: Color) -> Piece {
        if color == Color::White {
            Piece::new(self.0)
        } else {
            Piece::new(self.bits() + PIECE_TYPES_COUNT)
        }
    }
    pub fn char(self) -> char {
        PIECE_CHARS[self.0 as usize] as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

impl Debug for PieceType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0 {
            0 => write!(f, "pawn"),
            1 => write!(f, "knight"),
            2 => write!(f, "bishop"),
            3 => write!(f, "rook"),
            4 => write!(f, "queen"),
            5 => write!(f, "king"),
            16 => write!(f, "unknown"),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use colored_squares::*;
    use pieces::*;

    #[test]
    fn of_color() {
        assert_eq!(PAWN.of(Color::White),      WHITE_PAWN            );
        assert_eq!(KNIGHT.of(Color::White),    WHITE_KNIGHT                 );
        assert_eq!(BISHOP.of(Color::White),    WHITE_BISHOP                 );
        assert_eq!(ROOK.of(Color::White),      WHITE_ROOK             );
        assert_eq!(QUEEN.of(Color::White),     WHITE_QUEEN                );
        assert_eq!(KING.of(Color::White),      WHITE_KING             );
        assert_eq!(PAWN.of(Color::Black),      BLACK_PAWN              );
        assert_eq!(KNIGHT.of(Color::Black),    BLACK_KNIGHT                 );
        assert_eq!(BISHOP.of(Color::Black),    BLACK_BISHOP                 );
        assert_eq!(ROOK.of(Color::Black),      BLACK_ROOK              );
        assert_eq!(QUEEN.of(Color::Black),     BLACK_QUEEN                );
        assert_eq!(KING.of(Color::Black),      BLACK_KING             );
    }
}
