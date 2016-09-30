#![allow(dead_code)]

pub static PIECE_CHARS: &'static [u8; 12] = b"PNBRQKpnbrqk";

pub mod piece_types {
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::fmt::Result;
    use sqares::*;
    use super::pieces::*;

    pub const PIECE_TYPES_COUNT: i32 = 6;

    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    pub struct PieceType(i32);

    impl PieceType {
        pub fn new(bits: i32) -> Self {
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
            super::PIECE_CHARS[self.0 as usize] as char
        }
        pub fn bits(self) -> i32 {
            self.0
        }
    }

    impl Debug for PieceType {
        fn fmt(&self, f: &mut Formatter) -> Result {
            match self.0 {
                - 1 => write!(f, "unknown"),
                0 => write!(f, "pawn"),
                1 => write!(f, "knight"),
                2 => write!(f, "bishop"),
                3 => write!(f, "rook"),
                4 => write!(f, "queen"),
                5 => write!(f, "king"),
                _ => panic!(),
            }
        }
    }

    pub const PAWN: PieceType = PieceType(0);
    pub const KNIGHT: PieceType = PieceType(1);
    pub const BISHOP: PieceType = PieceType(2);
    pub const ROOK: PieceType = PieceType(3);
    pub const QUEEN: PieceType = PieceType(4);
    pub const KING: PieceType = PieceType(5);
    pub const UNKNOWN: PieceType = PieceType(16);
}

pub mod pieces {
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::fmt::Result;
    use sqares::*;
    use super::piece_types::*;
    use super::*;

    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    pub struct Piece(i32);

    pub const PIECES_COUNT: usize = 12;

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
    pub const EMPTY: Piece = Piece(-1);

    impl Piece {
        pub fn new(bits: i32) -> Self {
            Piece(bits)
        }
        pub fn bits(self) -> i32 {
            self.0
        }

        pub fn get_color(&self) -> Color {
            if self.0 >= PIECE_TYPES_COUNT {
                Color::Black
            } else {
                Color::White
            }
        }
        pub fn get_type(&self) -> PieceType {
            PieceType::new(self.bits() % PIECE_TYPES_COUNT)
        }
        pub fn as_char(&self) -> char {
            PIECE_CHARS[self.0 as usize] as char
        }
    }

    impl Debug for Piece {
        fn fmt(&self, f: &mut Formatter) -> Result {
            if *self == pieces::EMPTY {
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

    pub struct PieceIter(i32);

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

}

use self::piece_types::*;
use sqares::*;

const MOVE_FROM_MASK: u16 = 0b0000_0000_0000_1111;
const MOVE_TO_MASK: u16 = 0b0000_0000_1111_0000;
const MOVE_PROMOTE_TO_MASK: u16 = 0b0000_0011_0000_0000;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move(u16);

impl Move {
    pub fn usual(from: Square64, to: Square64) -> Self {
        Move::with_promotion(from, to, piece_types::UNKNOWN)
    }
    pub fn with_promotion(from: Square64, to: Square64, promote_to: PieceType) -> Self {
        Move((from.bits() as u16) | ((to.bits() as u16) << 4) | ((promote_to.bits() as u16) << 8))
    }
    pub fn from(self) -> Square64 {
        Square64::new((self.0 & MOVE_FROM_MASK) as u8)
    }
    pub fn to(self) -> Square64 {
        Square64::new(((self.0 & MOVE_TO_MASK) >> 4) as u8)
    }
    pub fn promote_to(self) -> PieceType {
        PieceType::new((((self.0 as u16) & MOVE_PROMOTE_TO_MASK) >> 8) as i32)
    }
    pub fn string(self) -> String {
        let mut result = String::with_capacity(6);
        result.push_str(self.from().to_string().as_str());
        result.push('-');
        result.push_str(self.to().to_string().as_str());
        let promote_to = self.promote_to();
        if promote_to != piece_types::UNKNOWN {
            result.push('=');
            result.push(promote_to.char());
        }
        result
    }
}


#[cfg(test)]
mod test {
    use sqares::*;
    use super::*;
    use super::pieces::*;
    use super::piece_types::*;
    use std::iter::*;

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
        let all = AllPieces.into_iter()
            .collect::<Vec<Piece>>();
        assert_eq!(all.len(), 12);
        assert_eq!(all[0], WHITE_PAWN);
        assert_eq!(all[11], BLACK_KING);
    }

    #[test]
    fn usual_move() {
        let e2 = Square64::parse("e2");
        let e4 = Square64::parse("e4");
        let m = Move::usual(e2, e4);
        assert_eq!(m.0, 0);
        assert_eq!(m.from().to_string(), "e2");
    }
    #[test]
    fn usual_move_to_string() {
        let e2 = Square64::parse("e2");
        let e4 = Square64::parse("e4");
        assert_eq!(Move::usual(e2, e4).string(), "e2-e4");
    }
    #[test]
    fn promotion_move_to_string() {
        let e2 = Square64::parse("e2");
        let e4 = Square64::parse("e4");
        assert_eq!(Move::with_promotion(e2, e4, piece_types::QUEEN).string(), "e2-e4=Q");
    }
}
