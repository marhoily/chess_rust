use std::fmt::{Result, Display, Formatter};
use castle::*;
use rank::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn en_passant_rank(self) -> Rank {
        if self == Color::White { _6 } else { _3 }
    }
    pub fn invert(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn castle(self) -> Castle {
        match self {
            Color::White => W,
            Color::Black => B,
        }
    }
    pub fn parse(input: char) -> Self {
        debug_assert!((input as u32) < 128, "it is not even an ASCII character!");
        parse_color(&[input as u8]).unwrap().1
    }
    pub fn char(self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}
named!(pub parse_color(&[u8]) -> Color,
    alt!(
        value!(Color::White, char!('w')) |
        value!(Color::Black, char!('b'))));

#[cfg(test)]
mod test {
    use super::*;
    use super::Color::*;

    #[test]
    fn color_invert() {
        assert_eq!(White.invert(), Black);
        assert_eq!(Black.invert(), White);
    }
    #[test]
    fn castle() {
        assert_eq!(White.castle(), W);
        assert_eq!(Black.castle(), B);
    }
    #[test]
    fn parse() {
        assert_eq!(Color::parse('w'), White);
        assert_eq!(Color::parse('b'), Black);
    }
    #[test]
    fn display() {
        assert_eq!(format!("{}", White), "w");
        assert_eq!(format!("{}", Black), "b");
    }
    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", White), "White");
        assert_eq!(format!("{:?}", Black), "Black");
    }
}
