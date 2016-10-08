#![allow(dead_code)]

use std::fmt::{Result, Display, Formatter};
use file::{File, parse_file};
use rank::{Rank, parse_rank};
use color::Color;
use color::Color::*;
use square::Square;
use mask::Mask;
use itertools::*;
pub use self::squares::*;

pub mod ops;
mod squares;

// Note that index 0 corresponds to a8, and NOT a1!
// Indexes read left to right, top to bottom!
#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Square88(u8);

impl Square88 {
    pub fn into_mask(self) -> Mask {
        self.into_square().mask()
    }
    pub fn into_square(self) -> Square {
        let (file, rank) = self.file_rank();
        Square::from(file, rank)
    }
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
    pub fn file(self) -> File {
        File::from_bits(self.0 & 7)
    }
    pub fn rank(self) -> Rank {
        Rank::from_bits(self.0 >> 4)
    }
    pub fn file_rank(self) -> (File, Rank) {
        (self.file(), self.rank())
    }
    pub fn color(self) -> Color {
        let (file, rank) = self.file_rank();
        if (file.bits() % 2) == (rank.bits() % 2) {
            White
        } else {
            Black
        }
    }
    pub fn is_valid(&self) -> bool {
        self.0 & 0x88 == 0
    }
    pub fn too_big(&self) -> bool {
        self.0 > 0x77
    }
    pub fn forward(self, offset: u8) -> Self {
        let mut result = Square88(self.0 + offset);
        while !result.too_big() && !result.is_valid() {
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

#[test]
fn forward() {
    let mut last = 0;
    for offset in 1..119 {
        let curr = ALL_SQUARES.forward(offset).bits();
        if curr < last {
            panic!("+{} -> {};\r\n+{} -> {}",
                    offset-1, ALL_SQUARES.forward(offset-1).bits(),
                    offset, ALL_SQUARES.forward(offset).bits());
        }
        last = curr;
    }
    // assert_eq!((0..120).map(|offset| ALL_SQUARES
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
    assert_eq!([H8, G7, F6, E5, D4, C3, B2, A1].into_iter().
            map(|s| format!("{}", s)).collect::<Vec<_>>(),
            ["h8","g7","f6","e5","d4","c3","b2","a1"]);
}

#[test]
fn square_debug() {
    assert_eq!([A8, H8, A1, H1].into_iter().
            map(|s| format!("{:?}", s)).collect_vec(),
            ["Square88(0)", "Square88(7)", "Square88(112)", "Square88(119)"]);
}

#[test]
fn square_parse() {
    assert_eq!(["a8","b7","c6","d5","e4","f3","g2","h1"].into_iter().
            map(|f| Square88::parse(*f)).collect_vec(),
            [A8, B7, C6, D5, E4, F3, G2, H1]);
}

#[test]
fn square_file_rank() {
    assert_equal(ALL_SQUARES.into_iter(),
            ALL_SQUARES.into_iter().map(|square| {
                    Square88::from(square.file(), square.rank())
                }));
}
// noinspection SpellCheckingInspection
#[test]
fn square_color() {
    assert_eq!(ALL_SQUARES.into_iter().map(|square| {
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
    for rank in ::rank::ALL_RANKS {
        for file in ::file::ALL_FILES {
            let square88 = Square88::from(file, rank);
            println!("pub const {}{}: Square88 = Square88(0x{:X});",
                         file.char().to_uppercase().last().unwrap(), rank, square88.0);
        }
    }
}
