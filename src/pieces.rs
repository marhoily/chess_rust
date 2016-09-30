use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use colored_squares::*;
use piece_types;
use piece_types::{PieceType};

pub const COUNT: usize = 12;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Piece(u8);

pub const WHITE_PAWN: Piece = Piece(0);
pub const WHITE_KNIGHT: Piece = Piece(1);
pub const WHITE_BISHOP: Piece = Piece(2);
pub const WHITE_ROOK: Piece = Piece(3);
pub const WHITE_QUEEN: Piece = Piece(4);
pub const WHITE_KING: Piece = Piece(5);
pub const BLACK_PAWN: Piece = Piece(6);
pub const BLACK_KNIGHT: Piece = Piece(7);
pub const BLACK_BISHOP: Piece = Piece(8);
pub const BLACK_ROOK: Piece = Piece(9);
pub const BLACK_QUEEN: Piece = Piece(10);
pub const BLACK_KING: Piece = Piece(11);
pub const EMPTY: Piece = Piece(16);

impl Piece {
    pub fn new(bits: u8) -> Self {
        Piece(bits)
    }
    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn get_color(&self) -> Color {
        if self.0 >= piece_types::COUNT {
            Color::Black
        } else {
            Color::White
        }
    }
    pub fn get_type(&self) -> PieceType {
        if *self == EMPTY {
            piece_types::UNKNOWN
        } else {
            PieceType::new(self.bits() % piece_types::COUNT)
        }
    }
    pub fn as_char(&self) -> char {
        SYMBOLS[self.0 as usize] as char
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self == EMPTY {
            write!(f, "Empty")
        } else {
            write!(f, "{:?}-{:?}", self.get_color(), self.get_type())
        }
    }
}

pub struct AllPieces;

impl IntoIterator for AllPieces {
    type Item = Piece;
    type IntoIter = PieceIter;

    fn into_iter(self) -> Self::IntoIter {
        PieceIter(0)
    }
}

pub struct PieceIter(u8);

impl Iterator for PieceIter {
    type Item = Piece;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < 12 {
            let result = Piece(self.0);
            self.0 += 1;
            Some(result)
        } else {
            None
        }
    }
}

static SYMBOLS: &'static [u8; 12] = b"PNBRQKpnbrqk";

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::*;
    use colored_squares::*;
    use piece_types::{
        PAWN,
        KNIGHT,
        BISHOP,
        ROOK,
        QUEEN,
        KING,
        UNKNOWN
    };

    #[test]
    fn piece_get_color() {
        assert_eq!(WHITE_PAWN.get_color(), Color::White);
        assert_eq!(WHITE_KNIGHT.get_color(), Color::White);
        assert_eq!(WHITE_BISHOP.get_color(), Color::White);
        assert_eq!(WHITE_ROOK.get_color(), Color::White);
        assert_eq!(WHITE_QUEEN.get_color(), Color::White);
        assert_eq!(WHITE_KING.get_color(), Color::White);
        assert_eq!(BLACK_PAWN.get_color(), Color::Black);
        assert_eq!(BLACK_KNIGHT.get_color(), Color::Black);
        assert_eq!(BLACK_BISHOP.get_color(), Color::Black);
        assert_eq!(BLACK_ROOK.get_color(), Color::Black);
        assert_eq!(BLACK_QUEEN.get_color(), Color::Black);
        assert_eq!(BLACK_KING.get_color(), Color::Black);
    }


    #[test]
    fn piece_get_type() {
        assert_eq!(EMPTY.get_type(), UNKNOWN);
        assert_eq!(WHITE_PAWN.get_type(), PAWN);
        assert_eq!(WHITE_KNIGHT.get_type(), KNIGHT);
        assert_eq!(WHITE_BISHOP.get_type(), BISHOP);
        assert_eq!(WHITE_ROOK.get_type(), ROOK);
        assert_eq!(WHITE_QUEEN.get_type(), QUEEN);
        assert_eq!(WHITE_KING.get_type(), KING);
        assert_eq!(BLACK_PAWN.get_type(), PAWN);
        assert_eq!(BLACK_KNIGHT.get_type(), KNIGHT);
        assert_eq!(BLACK_BISHOP.get_type(), BISHOP);
        assert_eq!(BLACK_ROOK.get_type(), ROOK);
        assert_eq!(BLACK_QUEEN.get_type(), QUEEN);
        assert_eq!(BLACK_KING.get_type(), KING);
    }

    #[test]
    fn piece_type_fmt() {
        assert_eq!(format!("{:?}", UNKNOWN), "unknown");
        assert_eq!(format!("{:?}", PAWN), "pawn");
        assert_eq!(format!("{:?}", KNIGHT), "knight");
        assert_eq!(format!("{:?}", BISHOP), "bishop");
        assert_eq!(format!("{:?}", ROOK), "rook");
        assert_eq!(format!("{:?}", QUEEN), "queen");
        assert_eq!(format!("{:?}", KING), "king");
    }

    #[test]
    fn piece_fmt() {
        assert_eq!(format!("{:?}", EMPTY), "Empty");
        assert_eq!(format!("{:?}", WHITE_PAWN), "White-pawn");
        assert_eq!(format!("{:?}", WHITE_KNIGHT), "White-knight");
        assert_eq!(format!("{:?}", WHITE_BISHOP), "White-bishop");
        assert_eq!(format!("{:?}", WHITE_ROOK), "White-rook");
        assert_eq!(format!("{:?}", WHITE_QUEEN), "White-queen");
        assert_eq!(format!("{:?}", WHITE_KING), "White-king");
        assert_eq!(format!("{:?}", BLACK_PAWN), "Black-pawn");
        assert_eq!(format!("{:?}", BLACK_KNIGHT), "Black-knight");
        assert_eq!(format!("{:?}", BLACK_BISHOP), "Black-bishop");
        assert_eq!(format!("{:?}", BLACK_ROOK), "Black-rook");
        assert_eq!(format!("{:?}", BLACK_QUEEN), "Black-queen");
        assert_eq!(format!("{:?}", BLACK_KING), "Black-king");
    }

    #[test]
    fn all_pieces() {
        let all = AllPieces.into_iter().collect::<Vec<Piece>>();
        assert_eq!(all.len(), 12);
        assert_eq!(all[0], WHITE_PAWN);
        assert_eq!(all[11], BLACK_KING);
    }
}
