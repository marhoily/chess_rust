use colored_square::{Square};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Mask(u64);

impl Mask {
    pub fn new(exp: u64) -> Self {
        Mask(exp)
    }
    pub fn square(square: Square) -> Self {
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
    pub fn with(&mut self, another: Mask) -> Mask {
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
pub mod masks {
    use super::Mask;

    pub const EMPTY: Mask = Mask(0);

    pub mod files {
        use super::super::Mask;

        pub const A : Mask = Mask(0x1010101010101);
        pub const B : Mask = Mask(0x2020202020202);
        pub const C : Mask = Mask(0x4040404040404);
        pub const D : Mask = Mask(0x8080808080808);
        pub const E : Mask = Mask(0x10101010101010);
        pub const F : Mask = Mask(0x20202020202020);
        pub const G : Mask = Mask(0x40404040404040);
    }
}


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

    #[test]
    fn print_const_files() {
        use colored_square::{File, Rank, Square};
        use std::ascii::AsciiExt;

        println!("");

        for file in 0..7 {
            let mut mask = masks::EMPTY;
            let f = File::new(file);
            for rank in 0..7 {
                let sq = Square::from(f, Rank::new(rank));
                mask = mask.with(sq.to_mask());
            }
            println!("pub const {} : Mask = Mask(0x{:X});", f.char().to_ascii_uppercase(), mask.bits());
        }
        println!("");
    }

}