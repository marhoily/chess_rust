#![allow(dead_code)]

struct File(i8);
struct Rank(i8);
struct Index64(i8);

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;


#[derive(PartialEq, PartialOrd)]
pub struct Piece(i32);
#[derive(PartialEq, PartialOrd)]
pub struct PieceType(i32);

impl Debug for PieceType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0 {
            -1 => write!(f, "unknown"),
            0 => write!(f, "pawn"),
            1 => write!(f, "knight"),
            2 => write!(f, "bishop"),
            3 => write!(f, "rook"),
            4 => write!(f, "queen"),
            5 => write!(f, "king"),
            _ => panic!()
        }
    }
}

#[derive(Debug)]
pub enum Color {
    Black,
    White,
}
const TYPES_COUNT : i32 = 6;

impl Piece {
    pub fn get_color(&self) -> Color {
        if self.0 >= TYPES_COUNT {
            Color::Black
        } else {
            Color::White
        }
    }
    pub fn get_type(&self) -> PieceType {
        PieceType(self.0 % TYPES_COUNT)
    }
}
impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self == EMPTY {
            write!(f, "Empty")
        }else {
            write!(f, "{:?}-{:?}", self.get_color(), self.get_type())
        }
    }
}

const WHITE_PAWN: Piece = Piece(0);
const WHITE_KNIGHT: Piece = Piece(1);
const WHITE_BISHOP: Piece = Piece(2);
const WHITE_ROOK: Piece = Piece(3);
const WHITE_QUEEN: Piece = Piece(4);
const WHITE_KING: Piece = Piece(5);
const BLACK_PAWN: Piece = Piece(6);
const BLACK_KNIGHT: Piece = Piece(7);
const BLACK_BISHOP: Piece = Piece(8);
const BLACK_ROOK: Piece = Piece(9);
const BLACK_QUEEN: Piece = Piece(10);
const BLACK_KING: Piece = Piece(11);
const EMPTY: Piece = Piece(-1);

pub struct BitBoard([u64; 12]);

impl BitBoard {
    fn new() -> Self {
        BitBoard([0; 12])
    }
    fn check_square(&self, square: Index64) -> Piece {
        for piece in 0..12 {
            if self.0[piece] & (1 << square.0) != 0 {
                return Piece(piece as i32);
            }
        }
        EMPTY
    }
}
#[cfg(test)]
mod test {
    use super::BitBoard;
    use super::Index64;
    use super::EMPTY;

    #[test]
    fn piece_partial_eq() {
        assert_eq!(super::EMPTY == super::EMPTY, true);
    }

    #[test]
    fn piece_fmt() {
        assert_eq!(format!("{:?}", super::EMPTY), "Empty");
        assert_eq!(format!("{:?}", super::WHITE_PAWN), "White-pawn");
        assert_eq!(format!("{:?}", super::WHITE_KNIGHT), "White-knight");
        assert_eq!(format!("{:?}", super::WHITE_BISHOP), "White-bishop");
        assert_eq!(format!("{:?}", super::WHITE_ROOK), "White-rook");
        assert_eq!(format!("{:?}", super::WHITE_QUEEN), "White-queen");
        assert_eq!(format!("{:?}", super::WHITE_KING), "White-king");
        assert_eq!(format!("{:?}", super::BLACK_PAWN), "Black-pawn");
        assert_eq!(format!("{:?}", super::BLACK_KNIGHT), "Black-knight");
        assert_eq!(format!("{:?}", super::BLACK_BISHOP), "Black-bishop");
        assert_eq!(format!("{:?}", super::BLACK_ROOK), "Black-rook");
        assert_eq!(format!("{:?}", super::BLACK_QUEEN), "Black-queen");
        assert_eq!(format!("{:?}", super::BLACK_KING), "Black-king");
    }
    #[test]
    fn basics() {
        let b = BitBoard::new();
        assert_eq!(b.check_square(Index64(0)), EMPTY);
    }
}
