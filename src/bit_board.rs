#![allow(dead_code)]

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

struct File(i8);
struct Rank(i8);

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
struct Square64(i8);

#[derive(PartialEq, Copy, Clone)]
struct SquareExp(u64);

impl Square64 {
    fn to_exp(&self) -> SquareExp {
        SquareExp(1 << self.0)
    }
}
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Piece(i32);
#[derive(PartialEq, PartialOrd, Copy, Clone)]
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
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}
const PIECE_TYPES_COUNT: i32 = 6;
const PIECES_COUNT: usize = 12;

impl Piece {
    pub fn get_color(&self) -> Color {
        if self.0 >= PIECE_TYPES_COUNT {
            Color::Black
        } else {
            Color::White
        }
    }
    pub fn get_type(&self) -> PieceType {
        PieceType(self.0 % PIECE_TYPES_COUNT)
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

const PAWN: PieceType = PieceType(0);
const KNIGHT: PieceType = PieceType(1);
const BISHOP: PieceType = PieceType(2);
const ROOK: PieceType = PieceType(3);
const QUEEN: PieceType = PieceType(4);
const KING: PieceType = PieceType(5);
const UNKNOWN: PieceType = PieceType(-1);

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

#[derive(PartialEq, Copy, Clone)]
pub struct PieceTypeBits(u64);
pub struct BitBoard([PieceTypeBits; PIECES_COUNT]);
impl PieceTypeBits {
    fn test(self, square: SquareExp) -> bool {
        self.0 & square.0 != 0
    }
    fn set(&mut self, square: SquareExp) {
        self.0 |= square.0
    }
}
struct AllPieces;

impl IntoIterator for AllPieces {
    type Item = Piece;
    type IntoIter = PieceIter;

    fn into_iter(self) -> Self::IntoIter {
        PieceIter(0)
    }
}
struct PieceIter(i32);
impl Iterator for PieceIter {
    type Item = Piece;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < 11 {
            self.0 += 1;
            Some(Piece(self.0))
        } else {
            None
        }
    }
}

impl BitBoard {
    fn new() -> Self {
        BitBoard([PieceTypeBits(0); PIECES_COUNT])
    }
    fn for_piece(&self, piece: Piece) -> PieceTypeBits {
        self.0[piece.0 as usize]
    }
    fn check_square(&self, square: SquareExp) -> Piece {
        for piece in AllPieces {
            if self.for_piece(piece).test(square) {
                return piece;
            }
        }
        EMPTY
    }
    fn set_piece(&mut self, square: SquareExp, piece: Piece) {
        self.0[piece.0 as usize].0 |= square.0;
    }
}
#[cfg(test)]
mod test {
    use super::BitBoard;
    use super::SquareExp;
    use super::Color;

    #[test]
    fn piece_get_color() {
        assert_eq!(super::WHITE_PAWN.get_color(), Color::White);
        assert_eq!(super::WHITE_KNIGHT.get_color(), Color::White);
        assert_eq!(super::WHITE_BISHOP.get_color(), Color::White);
        assert_eq!(super::WHITE_ROOK.get_color(), Color::White);
        assert_eq!(super::WHITE_QUEEN.get_color(), Color::White);
        assert_eq!(super::WHITE_KING.get_color(), Color::White);
        assert_eq!(super::BLACK_PAWN.get_color(), Color::Black);
        assert_eq!(super::BLACK_KNIGHT.get_color(), Color::Black);
        assert_eq!(super::BLACK_BISHOP.get_color(), Color::Black);
        assert_eq!(super::BLACK_ROOK.get_color(), Color::Black);
        assert_eq!(super::BLACK_QUEEN.get_color(), Color::Black);
        assert_eq!(super::BLACK_KING.get_color(), Color::Black);
    }

    #[test]
    fn piece_get_type() {
        assert_eq!(super::EMPTY.get_type(), super::UNKNOWN);
        assert_eq!(super::WHITE_PAWN.get_type(), super::PAWN);
        assert_eq!(super::WHITE_KNIGHT.get_type(), super::KNIGHT);
        assert_eq!(super::WHITE_BISHOP.get_type(), super::BISHOP);
        assert_eq!(super::WHITE_ROOK.get_type(), super::ROOK);
        assert_eq!(super::WHITE_QUEEN.get_type(), super::QUEEN);
        assert_eq!(super::WHITE_KING.get_type(), super::KING);
        assert_eq!(super::BLACK_PAWN.get_type(), super::PAWN);
        assert_eq!(super::BLACK_KNIGHT.get_type(), super::KNIGHT);
        assert_eq!(super::BLACK_BISHOP.get_type(), super::BISHOP);
        assert_eq!(super::BLACK_ROOK.get_type(), super::ROOK);
        assert_eq!(super::BLACK_QUEEN.get_type(), super::QUEEN);
        assert_eq!(super::BLACK_KING.get_type(), super::KING);
    }

    #[test]
    fn piece_type_fmt() {
        assert_eq!(format!("{:?}", super::UNKNOWN), "unknown");
        assert_eq!(format!("{:?}", super::PAWN), "pawn");
        assert_eq!(format!("{:?}", super::KNIGHT), "knight");
        assert_eq!(format!("{:?}", super::BISHOP), "bishop");
        assert_eq!(format!("{:?}", super::ROOK), "rook");
        assert_eq!(format!("{:?}", super::QUEEN), "queen");
        assert_eq!(format!("{:?}", super::KING), "king");
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
    fn check_square() {
        let mut b = BitBoard::new();
        b.set_piece(SquareExp(1), super::BLACK_ROOK);
        assert_eq!(b.check_square(SquareExp(1)), super::BLACK_ROOK);
    }
}
