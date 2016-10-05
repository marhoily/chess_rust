#![allow(dead_code)]

use std::fmt::{Result, Display, Formatter};
use file::{File, parse_file};
use rank::{Rank, parse_rank};
use color::Color;

// Note that index 0 corresponds to a8, and NOT a1!
// Indexes read left to right, top to bottom!
#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Square88(u8);

impl Square88 {
    pub fn new(bits: u8) -> Self {
        debug_assert!(bits < 120, "is not a valid square number");
        debug_assert!(bits & 0x88 != 0, "is not a valid square number");
        Square88(bits)
    }
    pub fn from(f: File, r: Rank) -> Self {
        Square88(f.bits() + r.bits() * 16)
    }
    pub fn parse(input: &str) -> Self {
        parse_square(input.as_bytes()).unwrap().1
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn file_rank(self) -> (File, Rank) {
        (File::new(self.0 & 7), Rank::new(self.0 >> 4))
    }
    pub fn color(self) -> Color {
        let (file, rank) = self.file_rank();
        if (file.bits() % 2) == (rank.bits() % 2) {
            Color::White
        } else {
            Color::Black
        }
    }
    pub fn is_valid(&self) -> bool {
        self.0 & 0x88 == 0 //&& self.0 < 0x77
    }
    pub fn forward(self, offset: u8) -> Self {
        let mut result = Square88(self.0 + offset);
        while result.0 < 0x77 && !result.is_valid() {
            result = Square88(result.0 + 1);
        }
        result
    }
}

impl Display for Square88 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let (file, rank) = self.file_rank();
        write!(f, "{}{}", file, rank)
    }
}

named!(pub parse_square(&[u8]) -> Square88,
    chain!(
        file: parse_file ~
        rank: parse_rank,
        || Square88::from(file, rank))
    );

pub mod ops;
pub mod squares;

#[cfg(test)]
mod test {
    use super::*;
    use super::squares::*;

    #[test]
    fn forward() {
        let mut last = 0;
        for offset in 1..119 {
            let curr = FIRST.forward(offset).bits();
            if curr < last {
                panic!("+{} -> {};\r\n+{} -> {}",
                    offset-1, FIRST.forward(offset-1).bits(),
                    offset, FIRST.forward(offset).bits());
            }
            last = curr;
        }
        // assert_eq!((0..120).map(|offset| FIRST
        //     .forward(offset))
        //     .collect::<Vec<_>>(),
        //     vec!( A8,B8,C8,D8,E8,F8,G8,H8,
        //             A7,B7,C7,D7,E7,F7,G7,H7,
        //             A6,B6,C6,D6,E6,F6,G6,H6,
        //             A5,B5,C5,D5,E5,F5,G5,H5,
        //             A4,B4,C4,D4,E4,F4,G4,H4,
        //             A3,B3,C3,D3,E3,F3,G3,H3,
        //             A2,B2,C2,D2,E2,F2,G2,H2,
        //             A1,B1,C1,D1,E1,F1,G1,H1 ));
        //        assert_eq !(FIRST.forward(120).is_valid(), false );
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
            ["Square88(0)", "Square88(7)", "Square88(112)", "Square88(119)"]);
    }

    #[test]
    fn square_parse() {
        use super::squares::*;

        assert_eq!(["a8","b7","c6","d5","e4","f3","g2","h1"].into_iter().
            map(|f| Square88::parse(*f)).collect::<Vec<_>>(),
            [A8, B7, C6, D5, E4, F3, G2, H1]);
    }

    #[test]
    fn square_file_rank() {
        use super::squares::*;
        assert_eq!(All.into_iter().collect::<Vec<_>>(),
            All.into_iter().map(|square| {
                    let (f, r) = square.file_rank();
                    Square88::from(f,r)
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

    #[test]
    fn print_const() {
        for rank in ::rank::ranks::All {
            for file in ::file::files::All {
                let square88 = Square88::from(file, rank);
                println!("pub const {}{}: Square88 = Square88(0x{:X});",
                         file.char().to_uppercase().last().unwrap(), rank, square88.0);
            }
        }
    }
}
