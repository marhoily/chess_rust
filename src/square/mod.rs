use std::fmt::{Result, Display, Formatter};
use mask::Mask;
use file::{File,parse_file};
use rank::{Rank,parse_rank};
use color::Color;

// Note that index 0 corresponds to a8, and NOT a1!
// Indexes read left to right, top to bottom!
#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Square(u8);

impl Square {
    pub fn new(bits: u8) -> Self {
       // debug_assert!(bits < 64, "is not a valid square number");
        Square(bits)
    }
    pub fn file(self) -> File {
        File::new(self.0 % 8)
    }
    pub fn rank(self) -> Rank {
        Rank::new(self.0 / 8)
    }
    pub fn from(f: File, r: Rank) -> Self {
        Square(f.bits() + r.bits() * 8)
    }
    pub fn parse(input: &str) -> Self {
        parse_square(input.as_bytes()).unwrap().1
    }
    pub fn mask(self) -> Mask {
        Mask::from(self)
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn file_rank(self) -> (File, Rank) {
        (self.file(), self.rank())
    }
    pub fn color(self) -> Color {
        let (file, rank) = self.file_rank();
        if (file.bits() % 2) == (rank.bits() % 2) {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let (file, rank) = self.file_rank();
        write!(f, "{}{}", file, rank)
    }
}

named!(pub parse_square(&[u8]) -> Square,
    chain!(
        file: parse_file ~
        rank: parse_rank,
        || Square::from(file, rank))
    );

pub mod squares;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn square_display() {
        use super::squares::*;

        assert_eq!([H8, G7, F6, E5, D4, C3, B2, A1].into_iter().
            map(|s| format!("{}", s)).collect::<Vec<_>>(),
            ["h8","g7","f6","e5","d4","c3","b2","a1"]);
    }
    #[test]
    fn square_debug() {
        use super::squares::*;

        assert_eq!([A8, H8, A1, H1].into_iter().
            map(|s| format!("{:?}", s)).collect::<Vec<_>>(),
            ["Square(0)", "Square(7)", "Square(56)", "Square(63)"]);
    }

    #[test]
    fn square_parse() {
        use super::squares::*;

        assert_eq!(["a8","b7","c6","d5","e4","f3","g2","h1"].into_iter().
            map(|f| Square::parse(*f)).collect::<Vec<_>>(),
            [A8, B7, C6, D5, E4, F3, G2, H1]);
    }

    #[test]
    fn square_file_rank() {
        use super::squares::*;
        assert_eq!(All.into_iter().collect::<Vec<_>>(),
            All.into_iter().map(|square| {
                    let (f, r) = square.file_rank();
                    Square::from(f,r)
                }).collect::<Vec<_>>());
    }
    // noinspection SpellCheckingInspection
    #[test]
    fn square_color() {
        use color::Color::*;
        use super::squares::*;
        assert_eq!(All.into_iter().map(|square| {
                    if square.color() == White { 'w' } else { 'b' }
                }).collect::<String>(),
                "wbwbwbwb\
                 bwbwbwbw\
                 wbwbwbwb\
                 bwbwbwbw\
                 wbwbwbwb\
                 bwbwbwbw\
                 wbwbwbwb\
                 bwbwbwbw");
    }
}
