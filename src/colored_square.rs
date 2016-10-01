use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Display;
use std::fmt::Formatter;
use mask::Mask;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
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

impl Debug for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

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

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

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

#[derive(Debug, PartialEq, Copy, Clone)]
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
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Square(u8);

impl Square {
    pub fn new(bits: u8) -> Self {
        Square(bits)
    }
    pub fn from(f: File, r: Rank) -> Self {
        Square(f.0 + r.0 * 8)
    }
    pub fn parse(input: &str) -> Self {
        parse_square(input.as_bytes()).unwrap().1
    }
    pub fn mask(self) -> Mask {
        Mask::square(self)
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
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(2);
        let (file, rank) = self.file_rank();
        result.push(file.char());
        result.push(rank.char());
        result
    }
}
impl Debug for Square {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
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
            map(|f| f.char()).collect::<Vec<char>>(),
            ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }
    #[test]
    fn rank_char() {
        use super::ranks::*;

        assert_eq!(All.into_iter().
            map(|f| f.char()).collect::<Vec<char>>(),
            ['8', '7', '6', '5', '4', '3', '2', '1']);
    }

    #[test]
    fn all_files() {
        use super::files::*;

        assert_eq!(
            All.into_iter().collect::<Vec<File>>(),
            [A, B, C, D, E, F, G, H]);
    }
    #[test]
    fn all_ranks() {
        use super::ranks::*;

        assert_eq!(
            All.into_iter().collect::<Vec<Rank>>(),
            [_8, _7, _6, _5, _4, _3, _2, _1]);
    }

    #[test]
    fn all_squares() {
        use super::squares::*;

        assert_eq!(All.into_iter().collect::<Vec<Square>>(), vec!(
            A8,B8,C8,D8,E8,F8,G8,H8,
            A7,B7,C7,D7,E7,F7,G7,H7,
            A6,B6,C6,D6,E6,F6,G6,H6,
            A5,B5,C5,D5,E5,F5,G5,H5,
            A4,B4,C4,D4,E4,F4,G4,H4,
            A3,B3,C3,D3,E3,F3,G3,H3,
            A2,B2,C2,D2,E2,F2,G2,H2,
            A1,B1,C1,D1,E1,F1,G1,H1 ));
    }

    #[test]
    fn file_parse() {
        use super::files::*;

        assert_eq!(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].into_iter().
            map(|f| File::parse(*f)).collect::<Vec<File>>(),
            [A, B, C, D, E, F, G, H]);
    }

    #[test]
    fn rank_parse() {
        use super::ranks::*;

        assert_eq!(['8', '7', '6', '5', '4', '3', '2', '1'].into_iter().
            map(|f| Rank::parse(*f)).collect::<Vec<Rank>>(),
            [_8, _7, _6, _5, _4, _3, _2, _1]);
    }

    #[test]
    fn square_parse() {
        use super::squares::*;

        assert_eq!(["a8","b7","c6","d5","e4","f3","g2","h1"].into_iter().
            map(|f| Square::parse(*f)).collect::<Vec<Square>>(),
            [A8, B7, C6, D5, E4, F3, G2, H1]);
    }

    #[test]
    fn square_to_string() {
        use super::squares::*;

        assert_eq!([H8, G7, F6, E5, D4, C3, B2, A1].into_iter().
            map(|s| s.to_string()).collect::<Vec<String>>(),
            ["h8","g7","f6","e5","d4","c3","b2","a1"]);
    }

    #[test]
    fn print_squares() {
        for s in squares::All {
            println!("pub const {} : Square = Square({});",
                     s.to_string().to_uppercase(), s.bits());
        }
    }
}
