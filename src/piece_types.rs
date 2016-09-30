use colored_squares::*;
use pieces::{Piece};

use std;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use nom::IResult;
use nom::IResult::*;

pub const COUNT: u8 = 6;

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
            Piece::new(self.bits() + COUNT)
        }
    }
    pub fn char(self) -> char {
        SYMBOLS[self.0 as usize] as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn parse(input: &str) -> Self {
        Self::try_parse(input).unwrap()
    }
    pub fn try_parse(input: &str) -> std::result::Result<Self, ParsePieceTypeError> {
        use nom::{Err, ErrorKind};
        match Self::parse_nom(input.as_bytes()) {
            Done(_, square) => Ok(square),
            Error(Err::Position(ErrorKind::Custom(code), _)) => Err(code),
            Incomplete(_) => Err(ParsePieceTypeError::Incomplete),
            _ => panic!("custom error!?")
        }
    }
    pub fn parse_nom(input: &[u8]) -> IResult<&[u8], Self, ParsePieceTypeError> {
        use nom::{Err, ErrorKind, Needed};
        if input.len() < 1 {
            return Incomplete(Needed::Size(1))
        }

        match input[0] as char {
            'P' => Done(&input[1..], PAWN),
            'N' => Done(&input[1..], KNIGHT),
            'B' => Done(&input[1..], BISHOP),
            'R' => Done(&input[1..], ROOK),
            'Q' => Done(&input[1..], QUEEN),
            'K' => Done(&input[1..], KING),
            _ => Error(Err::Position(ErrorKind::Custom(
                ParsePieceTypeError::Unrecognized), &input[1..]))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParsePieceTypeError {
    Unrecognized,
    Incomplete,
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

static SYMBOLS: &'static [u8; 6] = b"PNBRQK";

#[cfg(test)]
mod test {
    use super::*;
    use colored_squares::*;
    use pieces::{
        WHITE_PAWN,
        WHITE_KNIGHT,
        WHITE_BISHOP,
        WHITE_ROOK,
        WHITE_QUEEN,
        WHITE_KING,
        BLACK_PAWN,
        BLACK_KNIGHT,
        BLACK_BISHOP,
        BLACK_ROOK,
        BLACK_QUEEN,
        BLACK_KING
    };

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
    fn parse() {
        assert_eq!(PieceType::parse("P"), PAWN);
        assert_eq!(PieceType::parse("N"), KNIGHT);
        assert_eq!(PieceType::parse("B"), BISHOP);
        assert_eq!(PieceType::parse("R"), ROOK);
        assert_eq!(PieceType::parse("Q"), QUEEN);
        assert_eq!(PieceType::parse("K"), KING);
    }
}
