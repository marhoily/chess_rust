use geometry::*;
use piece::Piece;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct PieceType(u8);

pub mod piece_types {
    use super::PieceType;

    pub const COUNT: u8 = 6;

    pub const PAWN: PieceType = PieceType(0);
    pub const KNIGHT: PieceType = PieceType(1);
    pub const BISHOP: PieceType = PieceType(2);
    pub const ROOK: PieceType = PieceType(3);
    pub const QUEEN: PieceType = PieceType(4);
    pub const KING: PieceType = PieceType(5);
    pub const UNKNOWN: PieceType = PieceType(16);

    pub struct All;
    impl IntoIterator for All {
        type Item = PieceType;
        type IntoIter = PieceType;

        fn into_iter(self) -> Self::IntoIter {
            PieceType(0)
        }
    }
    impl Iterator for PieceType {
        type Item = PieceType;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 < COUNT {
                let result = *self;
                self.0 += 1;
                Some(result)
            } else {
                None
            }
        }
    }
}

impl PieceType {
    pub fn new(bits: u8) -> Self {
        PieceType(bits)
    }
    pub fn parse(input: &str) -> Self {
        parse_piece_type(input.as_bytes()).unwrap().1
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn of(self, color: Color) -> Piece {
        if color == Color::White {
            Piece::new(self.0)
        } else {
            Piece::new(self.bits() + piece_types::COUNT)
        }
    }
    pub fn char(self) -> char {
        SYMBOLS[self.0 as usize] as char
    }
}

named!(parse_piece_type(&[u8]) -> PieceType,
    map!(is_a!(SYMBOLS), |c: &[u8]| {
        PieceType(SYMBOLS.iter().position(|x| {
            *x == c[0]}).unwrap() as u8)}));

impl Display for PieceType {
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

static SYMBOLS: &'static [u8; 6] = b"PNBRQK";

#[cfg(test)]
mod test {
    use super::*;
    use super::piece_types::*;
    use geometry::*;
    use piece::pieces;

    #[test]
    fn of_color() {
        assert_eq!(PAWN.of(Color::White), pieces::WHITE_PAWN);
        assert_eq!(KNIGHT.of(Color::White), pieces::WHITE_KNIGHT);
        assert_eq!(BISHOP.of(Color::White), pieces::WHITE_BISHOP);
        assert_eq!(ROOK.of(Color::White), pieces::WHITE_ROOK);
        assert_eq!(QUEEN.of(Color::White), pieces::WHITE_QUEEN);
        assert_eq!(KING.of(Color::White), pieces::WHITE_KING);
        assert_eq!(PAWN.of(Color::Black), pieces::BLACK_PAWN);
        assert_eq!(KNIGHT.of(Color::Black), pieces::BLACK_KNIGHT);
        assert_eq!(BISHOP.of(Color::Black), pieces::BLACK_BISHOP);
        assert_eq!(ROOK.of(Color::Black), pieces::BLACK_ROOK);
        assert_eq!(QUEEN.of(Color::Black), pieces::BLACK_QUEEN);
        assert_eq!(KING.of(Color::Black), pieces::BLACK_KING);
    }

    #[test]
    fn all() {
        assert_eq!(All.into_iter().collect::<Vec<_>>(),
                   [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING]);
    }

    #[test]
    fn display() {
        assert_eq!([PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING, UNKNOWN]
                       .into_iter()
                       .map(|pt| format!("{}", pt))
                       .collect::<Vec<_>>(),
                   ["pawn", "knight", "bishop", "rook", "queen", "king", "unknown"]);
    }
    #[test]
    fn parse() {
        assert_eq!(PieceType::parse("P"), PAWN);
        assert_eq!(PieceType::parse("N"), KNIGHT);
        assert_eq!(PieceType::parse("B"), BISHOP);
        assert_eq!(PieceType::parse("R"), ROOK);
        assert_eq!(PieceType::parse("Q"), QUEEN);
        assert_eq!(PieceType::parse("K"), KING);
    }
}
