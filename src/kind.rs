use std::fmt::{Result, Display, Formatter};

use piece::Piece;
use color::Color;

#[derive(Eq, Copy, Clone, Debug, PartialEq, Hash)]
pub struct Kind(u8);

impl Kind {
    pub fn new(bits: u8) -> Self {
        Kind(bits)
    }
    pub fn parse(input: char) -> Self {
        debug_assert!((input as u32) < 128, "it is not even an ASCII character!");
        parse_kind(&[input as u8]).unwrap().1
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn of(self, color: Color) -> Piece {
        debug_assert!(self != UNKNOWN);
        if color == Color::White {
            Piece::new(self.0)
        } else {
            Piece::new(self.bits() + KINDS_COUNT)
        }
    }
    pub fn char(self) -> char {
        debug_assert!(self != UNKNOWN,
                      "There's no symbol defined for UNKNOWN");
        SYMBOLS[self.0 as usize] as char
    }
}

named!(parse_kind(&[u8]) -> Kind,
    map!(is_a!(SYMBOLS), |c: &[u8]| {
        Kind(SYMBOLS.iter().position(|x| {
            *x == c[0]}).unwrap() as u8)}));

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}

static SYMBOLS: &'static [u8; 6] = b"PNBRQK";

pub const KINDS_COUNT: u8 = 6;
pub const ALL_KINDS: Kind = Kind(0);

pub const PAWN: Kind = Kind(0);
pub const KNIGHT: Kind = Kind(1);
pub const BISHOP: Kind = Kind(2);
pub const ROOK: Kind = Kind(3);
pub const QUEEN: Kind = Kind(4);
pub const KING: Kind = Kind(5);
pub const UNKNOWN: Kind = Kind(16);

impl Iterator for Kind {
    type Item = Kind;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < KINDS_COUNT {
            let result = *self;
            self.0 += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::*;

    #[test]
    fn of_color() {
        use piece::*;
        use color::Color;

        let white = ALL_KINDS.map(|pt| pt.of(Color::White));
        let black = ALL_KINDS.map(|pt| pt.of(Color::Black));
        let together = white.chain(black).collect_vec();

        assert_eq!(together, ALL_PIECES.into_iter().collect_vec());
    }
    #[test]
    fn display() {
        assert_eq!(ALL_KINDS
                       .map(|pt| format!("{}", pt))
                       .collect::<String>(),
                   "PNBRQK");
    }
    #[test]
    fn debug() {
        assert_eq!([PAWN, KING, UNKNOWN]
                       .into_iter()
                       .map(|pt| format!("{:?}", pt))
                       .collect_vec(),
                   ["Kind(0)", "Kind(5)", "Kind(16)"]);
    }
    // noinspection SpellCheckingInspection
    #[test]
    fn parse() {
        assert_eq!("PNBRQK"
                       .chars()
                       .into_iter()
                       .map(Kind::parse)
                       .collect_vec(),
                   [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING]);
    }
}
