use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Display;
use std::fmt::Formatter;
use mask::Mask;

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
pub struct Square(u8);

impl Square {
    pub fn new(square_number: u8) -> Self {
        Square(square_number)
    }
    pub fn from(f: File, r: Rank) -> Self {
        Square(f.0 + r.0 * 8)
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

named!(pub parse_square(&[u8]) -> Square,
    chain!(
        file: parse_file ~
        rank: parse_rank,
        || Square::from(file, rank))
    );

pub mod squares {
    use super::Square;
    pub const FIRST: Square = Square(0);

    pub const A8: Square = Square(0);
    pub const B8: Square = Square(1);
    pub const C8: Square = Square(2);
    pub const D8: Square = Square(3);
    pub const E8: Square = Square(4);
    pub const F8: Square = Square(5);
    pub const G8: Square = Square(6);
    pub const H8: Square = Square(7);
    pub const A7: Square = Square(8);
    pub const B7: Square = Square(9);
    pub const C7: Square = Square(10);
    pub const D7: Square = Square(11);
    pub const E7: Square = Square(12);
    pub const F7: Square = Square(13);
    pub const G7: Square = Square(14);
    pub const H7: Square = Square(15);
    pub const A6: Square = Square(16);
    pub const B6: Square = Square(17);
    pub const C6: Square = Square(18);
    pub const D6: Square = Square(19);
    pub const E6: Square = Square(20);
    pub const F6: Square = Square(21);
    pub const G6: Square = Square(22);
    pub const H6: Square = Square(23);
    pub const A5: Square = Square(24);
    pub const B5: Square = Square(25);
    pub const C5: Square = Square(26);
    pub const D5: Square = Square(27);
    pub const E5: Square = Square(28);
    pub const F5: Square = Square(29);
    pub const G5: Square = Square(30);
    pub const H5: Square = Square(31);
    pub const A4: Square = Square(32);
    pub const B4: Square = Square(33);
    pub const C4: Square = Square(34);
    pub const D4: Square = Square(35);
    pub const E4: Square = Square(36);
    pub const F4: Square = Square(37);
    pub const G4: Square = Square(38);
    pub const H4: Square = Square(39);
    pub const A3: Square = Square(40);
    pub const B3: Square = Square(41);
    pub const C3: Square = Square(42);
    pub const D3: Square = Square(43);
    pub const E3: Square = Square(44);
    pub const F3: Square = Square(45);
    pub const G3: Square = Square(46);
    pub const H3: Square = Square(47);
    pub const A2: Square = Square(48);
    pub const B2: Square = Square(49);
    pub const C2: Square = Square(50);
    pub const D2: Square = Square(51);
    pub const E2: Square = Square(52);
    pub const F2: Square = Square(53);
    pub const G2: Square = Square(54);
    pub const H2: Square = Square(55);
    pub const A1: Square = Square(56);
    pub const B1: Square = Square(57);
    pub const C1: Square = Square(58);
    pub const D1: Square = Square(59);
    pub const E1: Square = Square(60);
    pub const F1: Square = Square(61);
    pub const G1: Square = Square(62);
    pub const H1: Square = Square(63);

    pub struct All;
    impl IntoIterator for All {
        type Item = Square;
        type IntoIter = Square;

        fn into_iter(self) -> Self::IntoIter {
            Square(0)
        }
    }
    impl Iterator for Square {
        type Item = Square;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == 64 {
                None
            } else {
                let copy = *self;
                self.0 += 1;
                Some(copy)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::files::*;

    #[test]
    fn color_invert() {
        assert_eq!(Color::White.invert(), Color::Black);
        assert_eq!(Color::Black.invert(), Color::White);
    }

    #[test]
    fn file_char() {
        assert_eq!(All.into_iter().
            map(|f| f.char()).collect::<Vec<char>>(),
            ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }

    #[test]
    fn all_files() {
        assert_eq!(
            All.into_iter().collect::<Vec<File>>(),
            [A, B, C, D, E, F, G, H]);
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
        assert_eq!(Square::parse("a8").0, 0);
        assert_eq!(Square::parse("b7").0, 9);
        assert_eq!(Square::parse("c6").0, 18);
        assert_eq!(Square::parse("d5").0, 27);
        assert_eq!(Square::parse("e4").0, 36);
        assert_eq!(Square::parse("f3").0, 45);
        assert_eq!(Square::parse("g2").0, 54);
        assert_eq!(Square::parse("h1").0, 63);
    }

    #[test]
    fn square_to_string() {
        assert_eq!(Square::parse("a8").to_string(), "a8");
        assert_eq!(Square::parse("b7").to_string(), "b7");
        assert_eq!(Square::parse("c6").to_string(), "c6");
        assert_eq!(Square::parse("d5").to_string(), "d5");
        assert_eq!(Square::parse("e4").to_string(), "e4");
        assert_eq!(Square::parse("f3").to_string(), "f3");
        assert_eq!(Square::parse("g2").to_string(), "g2");
        assert_eq!(Square::parse("h1").to_string(), "h1");
    }

    #[test]
    fn print_const() {
        println!("");

        for s in squares::All {
            println!("pub const {} : Square = Square({});", s.to_string().to_uppercase(), s.bits());
        }
        println!("");
    }
}
