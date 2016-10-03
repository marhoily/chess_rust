use piece::Piece;
use color::Color;

use std::fmt::{Result, Display, Formatter};

#[derive(Eq, Copy, Clone, Debug, PartialEq, Hash)]
pub struct Kind(u8);

pub mod kinds {
    use super::Kind;

    pub const COUNT: u8 = 6;

    pub const PAWN: Kind = Kind(0);
    pub const KNIGHT: Kind = Kind(1);
    pub const BISHOP: Kind = Kind(2);
    pub const ROOK: Kind = Kind(3);
    pub const QUEEN: Kind = Kind(4);
    pub const KING: Kind = Kind(5);
    pub const UNKNOWN: Kind = Kind(16);

    #[derive(Copy, Clone, Debug)]
    pub struct All;
    impl IntoIterator for All {
        type Item = Kind;
        type IntoIter = Kind;

        fn into_iter(self) -> Self::IntoIter {
            Kind(0)
        }
    }
    impl Iterator for Kind {
        type Item = Kind;

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
        debug_assert!(self != kinds::UNKNOWN);
        if color == Color::White {
            Piece::new(self.0)
        } else {
            Piece::new(self.bits() + kinds::COUNT)
        }
    }
    pub fn char(self) -> char {
        debug_assert!(self != kinds::UNKNOWN,
                      "There's no symbol defined for kinds::UNKNOWN");
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

#[cfg(test)]
mod test {
    use super::*;
    use geometry::*;

    #[test]
    fn of_color() {
        use piece::pieces;
        use color::Color;

        let white = kinds::All.into_iter().map(|pt| pt.of(Color::White));
        let black = kinds::All.into_iter().map(|pt| pt.of(Color::Black));
        let together = white.chain(black).collect::<Vec<_>>();

        assert_eq!(together, pieces::All.into_iter().collect::<Vec<_>>());
    }
    #[test]
    fn display() {
        assert_eq!(super::kinds::All.into_iter()
                       .map(|pt| format!("{}", pt))
                       .collect::<String>(),
                   "PNBRQK");
    }
    #[test]
    fn debug() {
        use super::kinds::*;

        assert_eq!([PAWN, KING, UNKNOWN]
                       .into_iter()
                       .map(|pt| format!("{:?}", pt))
                       .collect::<Vec<_>>(),
                   ["Kind(0)", "Kind(5)", "Kind(16)"]);
    }
    // noinspection SpellCheckingInspection
    #[test]
    fn parse() {
        use super::kinds::*;

        assert_eq!("PNBRQK"
                       .chars()
                       .into_iter()
                       .map(Kind::parse)
                       .collect::<Vec<_>>(),
                   [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING]);
    }
}
