use std::fmt::{Result, Display, Formatter};
use castle::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn invert(self) -> Self {
        if self == Color::Black {
            Color::White
        } else {
            Color::Black
        }
    }
    pub fn castle(self) -> Castle {
        if self == Color::Black {
            B
        } else {
            W
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self == Color::White {
            write!(f, "w")
        } else {
            write!(f, "b")
        }
    }
}
named!(pub parse_color(&[u8]) -> Color,
    alt!(
        value!(Color::White, char!('w')) |
        value!(Color::Black, char!('b'))));

#[cfg(test)]
mod test {

    #[test]
    fn color_invert() {
        use super::Color::*;

        assert_eq!(White.invert(), Black);
        assert_eq!(Black.invert(), White);
    }

}