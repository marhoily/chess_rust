use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Display;
use std::fmt::Formatter;
use masks::Mask;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct File(u8);

impl File {
    pub fn new(num: u8) -> Self {
        File(num)
    }
    pub fn parse(input: char) -> Self {
        let mut str = String::with_capacity(1);
        str.push(input);
        parse_file(str.as_bytes()).unwrap().1
    }
    pub fn char(self) -> char {
        ('a' as u8 + self.0 as u8) as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

named!(parse_file(&[u8]) -> File,
    map!(is_a!("abcdefgh"),
        |c| File((c as &[u8])[0] - ('a' as u8))));

impl Debug for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Rank(u8);

impl Rank {
    pub fn new(num: u8) -> Self {
        Rank(num)
    }
    pub fn parse(input: char) -> Self {
        let mut str = String::with_capacity(1);
        str.push(input);
        parse_rank(str.as_bytes()).unwrap().1
    }
    pub fn char(self) -> char {
        ('8' as u8 - self.0 as u8) as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

named!(parse_rank(&[u8]) -> Rank,
    map!(is_a!("87654321"),
        |c| Rank(('8' as u8) - (c as &[u8])[0])));

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

#[derive(Debug, PartialEq)]
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
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Square64(u8);

impl Square64 {
    pub fn new(square_number: u8) -> Self {
        Square64(square_number)
    }
    pub fn from(f: File, r: Rank) -> Self {
        Square64(f.0 + r.0 * 8)
    }
    pub fn parse(input: &str) -> Self {
        parse_square(input.as_bytes()).unwrap().1
    }
    pub fn to_mask(&self) -> Mask {
        Mask::square(*self)
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn humanize(self) -> (File, Rank) {
        (File(self.0 % 8), Rank(self.0 / 8))
    }
    pub fn color(self) -> Color {
        let (file, rank) = self.humanize();
        if (file.0 % 2) == (rank.0 % 2) {
            Color::White
        } else {
            Color::Black
        }
    }
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(2);
        let (file, rank) = self.humanize();
        result.push(file.char());
        result.push(rank.char());
        result
    }
}

named!(pub parse_square(&[u8]) -> Square64,
    chain!(
        file: parse_file ~
        rank: parse_rank,
        || Square64::from(file, rank))
    );

pub mod squares {
    use super::{Square64};
    pub const FIRST: Square64 = Square64(0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_invert() {
        assert_eq!(Color::White.invert(), Color::Black);
        assert_eq!(Color::Black.invert(), Color::White);
    }

    #[test]
    fn file_char() {
        assert_eq!(File::new(0).char(), 'a');
        assert_eq!(File::new(1).char(), 'b');
        assert_eq!(File::new(2).char(), 'c');
        assert_eq!(File::new(3).char(), 'd');
        assert_eq!(File::new(4).char(), 'e');
        assert_eq!(File::new(5).char(), 'f');
        assert_eq!(File::new(6).char(), 'g');
        assert_eq!(File::new(7).char(), 'h');
    }

    #[test]
    fn rank_char() {
        assert_eq!(Rank::new(0).char(), '8');
        assert_eq!(Rank::new(1).char(), '7');
        assert_eq!(Rank::new(2).char(), '6');
        assert_eq!(Rank::new(3).char(), '5');
        assert_eq!(Rank::new(4).char(), '4');
        assert_eq!(Rank::new(5).char(), '3');
        assert_eq!(Rank::new(6).char(), '2');
        assert_eq!(Rank::new(7).char(), '1');
    }

    #[test]
    fn file_parse() {
        assert_eq!(File::parse('a').0, 0);
        assert_eq!(File::parse('b').0, 1);
        assert_eq!(File::parse('c').0, 2);
        assert_eq!(File::parse('d').0, 3);
        assert_eq!(File::parse('e').0, 4);
        assert_eq!(File::parse('f').0, 5);
        assert_eq!(File::parse('g').0, 6);
        assert_eq!(File::parse('h').0, 7);
    }

    #[test]
    fn rank_parse() {
        assert_eq!(Rank::parse('8').0, 0);
        assert_eq!(Rank::parse('7').0, 1);
        assert_eq!(Rank::parse('6').0, 2);
        assert_eq!(Rank::parse('5').0, 3);
        assert_eq!(Rank::parse('4').0, 4);
        assert_eq!(Rank::parse('3').0, 5);
        assert_eq!(Rank::parse('2').0, 6);
        assert_eq!(Rank::parse('1').0, 7);
    }

    #[test]
    fn square_parse() {
        assert_eq!(Square64::parse("a8").0, 0);
        assert_eq!(Square64::parse("b7").0, 9);
        assert_eq!(Square64::parse("c6").0, 18);
        assert_eq!(Square64::parse("d5").0, 27);
        assert_eq!(Square64::parse("e4").0, 36);
        assert_eq!(Square64::parse("f3").0, 45);
        assert_eq!(Square64::parse("g2").0, 54);
        assert_eq!(Square64::parse("h1").0, 63);
    }

    #[test]
    fn square_to_string() {
        assert_eq!(Square64::parse("a8").to_string(), "a8");
        assert_eq!(Square64::parse("b7").to_string(), "b7");
        assert_eq!(Square64::parse("c6").to_string(), "c6");
        assert_eq!(Square64::parse("d5").to_string(), "d5");
        assert_eq!(Square64::parse("e4").to_string(), "e4");
        assert_eq!(Square64::parse("f3").to_string(), "f3");
        assert_eq!(Square64::parse("g2").to_string(), "g2");
        assert_eq!(Square64::parse("h1").to_string(), "h1");
    }
}
