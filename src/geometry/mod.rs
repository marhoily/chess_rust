use std::fmt::Result;
use std::fmt::Display;
use std::fmt::Formatter;
use mask::Mask;

#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct File(u8);

impl File {
    pub fn new(bits: u8) -> Self {
        debug_assert!(bits < 8);
        File(bits)
    }
    pub fn parse(input: char) -> Self {
        debug_assert!((input as u32) < 128, "it is not even an ASCII character!");
        parse_file(&[input as u8]).unwrap().1
    }
    pub fn char(self) -> char {
        (FILES[0] + self.0) as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

static FILES: &'static [u8; 8] = b"abcdefgh";

named!(parse_file(&[u8]) -> File,
    map!(is_a!(FILES), |c: &[u8]| File(c[0] - FILES[0])));

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}

pub mod files {
    use super::File;

    pub const A: File = File(0);
    pub const B: File = File(1);
    pub const C: File = File(2);
    pub const D: File = File(3);
    pub const E: File = File(4);
    pub const F: File = File(5);
    pub const G: File = File(6);
    pub const H: File = File(7);

    #[derive(Copy, Clone, Debug)]
    pub struct All;

    impl IntoIterator for All {
        type Item = File;
        type IntoIter = File;

        fn into_iter(self) -> Self::IntoIter {
            File(0)
        }
    }
    impl Iterator for File {
        type Item = File;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == 8 {
                None
            } else {
                let result = *self;
                self.0 += 1;
                Some(result)
            }
        }
    }
}

#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Rank(u8);

impl Rank {
    pub fn new(bits: u8) -> Self {
        debug_assert!(bits < 8);
        Rank(bits)
    }
    pub fn parse(input: char) -> Self {
        debug_assert!((input as u32) < 128, "it is not even an ASCII character!");
        parse_rank(&[input as u8]).unwrap().1
    }
    pub fn char(self) -> char {
        (RANKS[0] - self.0) as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

static RANKS: &'static [u8; 8] = b"87654321";

named!(parse_rank(&[u8]) -> Rank,
    map!(is_a!(RANKS), |c:&[u8]| Rank(RANKS[0] - c[0])));

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}

pub mod ranks {
    use super::Rank;

    pub const _1: Rank = Rank(7);
    pub const _2: Rank = Rank(6);
    pub const _3: Rank = Rank(5);
    pub const _4: Rank = Rank(4);
    pub const _5: Rank = Rank(3);
    pub const _6: Rank = Rank(2);
    pub const _7: Rank = Rank(1);
    pub const _8: Rank = Rank(0);

    #[derive(Copy, Clone, Debug)]
    pub struct All;

    impl IntoIterator for All {
        type Item = Rank;
        type IntoIter = Rank;

        fn into_iter(self) -> Self::IntoIter {
            Rank(0)
        }
    }
    impl Iterator for Rank {
        type Item = Rank;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == 8 {
                None
            } else {
                let result = *self;
                self.0 += 1;
                Some(result)
            }
        }
    }
}

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
}

// Note that index 0 corresponds to a8, and NOT a1!
// Indexes read left to right, top to bottom!
#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Square(u8);

impl Square {
    pub fn new(bits: u8) -> Self {
        debug_assert!(bits < 64, "is not a valid square number");
        Square(bits)
    }
    pub fn from(f: File, r: Rank) -> Self {
        Square(f.0 + r.0 * 8)
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
        (File(self.0 % 8), Rank(self.0 / 8))
    }
    pub fn color(self) -> Color {
        let (file, rank) = self.file_rank();
        if (file.0 % 2) == (rank.0 % 2) {
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
    fn color_invert() {
        use super::Color::*;

        assert_eq!(White.invert(), Black);
        assert_eq!(Black.invert(), White);
    }

    #[test]
    fn file_char() {
        use super::files::*;

        assert_eq!(All.into_iter().
            map(|f| f.char()).collect::<Vec<_>>(),
            ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }
    #[test]
    fn file_display() {
        use super::files::*;

        assert_eq!(All.into_iter().
            map(|f| format!("{}", f)).collect::<Vec<_>>(),
            ["a", "b", "c", "d", "e", "f", "g", "h"]);
    }
    #[test]
    fn file_debug() {
        use super::files::*;

        assert_eq!([A, H].into_iter().
            map(|f| format!("{:?}", f)).collect::<Vec<_>>(),
            ["File(0)", "File(7)"]);
    }
    #[test]
    fn rank_char() {
        use super::ranks::*;

        assert_eq!(All.into_iter().
            map(|f| f.char()).collect::<Vec<_>>(),
            ['8', '7', '6', '5', '4', '3', '2', '1']);
    }
    #[test]
    fn rank_display() {
        use super::ranks::*;

        assert_eq!(All.into_iter().
            map(|f| format!("{}", f)).collect::<Vec<_>>(),
            ["8", "7", "6", "5", "4", "3", "2", "1"]);
    }
    #[test]
    fn rank_debug() {
        use super::ranks::*;

        assert_eq!([_1, _8].into_iter().
            map(|f| format!("{:?}", f)).collect::<Vec<_>>(),
            ["Rank(7)", "Rank(0)"]);
    }
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
    fn file_parse() {
        use super::files::*;

        assert_eq!(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].into_iter().
            map(|f| File::parse(*f)).collect::<Vec<_>>(),
            [A, B, C, D, E, F, G, H]);
    }
    #[test]
    fn rank_parse() {
        use super::ranks::*;

        assert_eq!(['8', '7', '6', '5', '4', '3', '2', '1'].into_iter().
            map(|f| Rank::parse(*f)).collect::<Vec<_>>(),
            [_8, _7, _6, _5, _4, _3, _2, _1]);
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
        use super::squares::*;
        assert_eq!(All.into_iter().map(|square| {
                    if square.color() == Color::White { 'w' } else { 'b' }
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
