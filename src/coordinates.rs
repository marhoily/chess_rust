#![allow(dead_code)]

use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct File(i8);
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Rank(i8);

impl File {
    pub fn new(num: i8) -> Self{
        File(num)
    }
    pub fn from(f: File, r: Rank) -> Self{
        File(f.0 + r.0*8)
    }
    pub fn char(self) -> char {
        ('a' as u8 + self.0 as u8) as char
    }
}
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
// Note that index 0 corresponds to a8, and NOT a1!
// Indexes read left to right, top to bottom!
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Square64(i8);

impl Square64 {
    pub fn new(square_number: i8) -> Self {
        Square64(square_number)
    }
    pub fn to_exp(&self) -> SquareExp {
        SquareExp(1 << self.0)
    }
    pub fn bits(self) -> i8 {
        self.0
    }
    pub fn humanize(self) -> (File, Rank) {
        (File(self.0 % 8), Rank(self.0 / 8))
    }
    //pub fn color(self) -> Color {
    //    let file, rank = Coordinate.FromIdx64 idx64
    //    if (file % 2) = (rank % 2) then Color.White
    //    else Color.Black
    //}
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct SquareExp(u64);

impl SquareExp {
    pub fn new(exp: u64) -> Self {
        SquareExp(exp)
    }
    pub fn bits(self) -> u64 {
        self.0
    }
    pub fn is_out(&self) -> bool {
        self.0 == 0
    }
    pub fn next(&mut self) {
        self.0 <<= 1;
    }
    pub fn forward(&mut self, count: u8) {
        self.0 <<= count;
    }
}

pub struct AllSquaresExp;

impl IntoIterator for AllSquaresExp {
    type Item = SquareExp;
    type IntoIter = SquareExpIter;

    fn into_iter(self) -> Self::IntoIter {
        SquareExpIter::new()
    }
}

pub struct SquareExpIter(u64);

impl SquareExpIter {
    pub fn new() -> Self {
        SquareExpIter(1)
    }
}

impl Iterator for SquareExpIter {
    type Item = SquareExp;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let result = SquareExp(self.0);
            self.0 <<= 1;
            Some(result)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::iter::*;

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
    fn all_squares_exp() {
        let all = AllSquaresExp.into_iter()
            .collect::<Vec<SquareExp>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], SquareExp(1));
        assert_eq!(all[63], SquareExp(1 << 63));
    }
}
