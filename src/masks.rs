use colored_squares::{Square64};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Mask(u64);

impl Mask {
    pub fn new(exp: u64) -> Self {
        Mask(exp)
    }
    pub fn square(square: Square64) -> Self {
        Mask(1 << square.bits())
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
    pub fn test(self, square: Mask) -> bool {
        self.0 & square.bits() != 0
    }
    pub fn union(&mut self, another: Mask) -> Mask {
        Mask(self.0 | another.0)
    }
}

pub struct AllSquaresExp;

impl IntoIterator for AllSquaresExp {
    type Item = Mask;
    type IntoIter = SquareMaskIter;

    fn into_iter(self) -> Self::IntoIter {
        SquareMaskIter::new()
    }
}

pub struct SquareMaskIter(u64);

impl SquareMaskIter {
    pub fn new() -> Self {
        SquareMaskIter(1)
    }
}

impl Iterator for SquareMaskIter {
    type Item = Mask;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let result = Mask(self.0);
            self.0 <<= 1;
            Some(result)
        }
    }
}

pub const EMPTY: Mask = Mask(0);


#[cfg(test)]
mod test {
    use super::*;
    use std::iter::*;

    #[test]
    fn all_squares_exp() {
        let all = AllSquaresExp.into_iter()
            .collect::<Vec<Mask>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], Mask(1));
        assert_eq!(all[63], Mask(1 << 63));
    }
}