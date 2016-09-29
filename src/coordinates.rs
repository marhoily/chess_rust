#![allow(dead_code)]

pub struct File(i8);
pub struct Rank(i8);

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
    fn all_squares_exp() {
        let all = AllSquaresExp.into_iter()
            .collect::<Vec<SquareExp>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], SquareExp(1));
        assert_eq!(all[63], SquareExp(1 << 63));
    }


}
