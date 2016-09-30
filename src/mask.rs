use colored_square::Square;

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

        pub const A: Mask = Mask(0x101010101010101);
        pub const B: Mask = Mask(0x202020202020202);
        pub const C: Mask = Mask(0x404040404040404);
        pub const D: Mask = Mask(0x808080808080808);
        pub const E: Mask = Mask(0x1010101010101010);
        pub const F: Mask = Mask(0x2020202020202020);
        pub const G: Mask = Mask(0x4040404040404040);
        pub const H: Mask = Mask(0x8080808080808080);

        pub static ALL: &'static [Mask] = &[A, B, C, D, E, F, G, H];
    }
    pub mod ranks {
        use super::super::Mask;

        pub const _8: Mask = Mask(0xFF);
        pub const _7: Mask = Mask(0xFF00);
        pub const _6: Mask = Mask(0xFF0000);
        pub const _5: Mask = Mask(0xFF000000);
        pub const _4: Mask = Mask(0xFF00000000);
        pub const _3: Mask = Mask(0xFF0000000000);
        pub const _2: Mask = Mask(0xFF000000000000);
        pub const _1: Mask = Mask(0xFF00000000000000);

        pub static ALL: &'static [Mask] = &[_1, _2, _3, _4, _5, _6, _7, _8];
    }

    pub const A8: Mask = Mask(0x1);
    pub const B8: Mask = Mask(0x2);
    pub const C8: Mask = Mask(0x4);
    pub const D8: Mask = Mask(0x8);
    pub const E8: Mask = Mask(0x10);
    pub const F8: Mask = Mask(0x20);
    pub const G8: Mask = Mask(0x40);
    pub const H8: Mask = Mask(0x80);
    pub const A7: Mask = Mask(0x100);
    pub const B7: Mask = Mask(0x200);
    pub const C7: Mask = Mask(0x400);
    pub const D7: Mask = Mask(0x800);
    pub const E7: Mask = Mask(0x1000);
    pub const F7: Mask = Mask(0x2000);
    pub const G7: Mask = Mask(0x4000);
    pub const H7: Mask = Mask(0x8000);
    pub const A6: Mask = Mask(0x10000);
    pub const B6: Mask = Mask(0x20000);
    pub const C6: Mask = Mask(0x40000);
    pub const D6: Mask = Mask(0x80000);
    pub const E6: Mask = Mask(0x100000);
    pub const F6: Mask = Mask(0x200000);
    pub const G6: Mask = Mask(0x400000);
    pub const H6: Mask = Mask(0x800000);
    pub const A5: Mask = Mask(0x1000000);
    pub const B5: Mask = Mask(0x2000000);
    pub const C5: Mask = Mask(0x4000000);
    pub const D5: Mask = Mask(0x8000000);
    pub const E5: Mask = Mask(0x10000000);
    pub const F5: Mask = Mask(0x20000000);
    pub const G5: Mask = Mask(0x40000000);
    pub const H5: Mask = Mask(0x80000000);
    pub const A4: Mask = Mask(0x100000000);
    pub const B4: Mask = Mask(0x200000000);
    pub const C4: Mask = Mask(0x400000000);
    pub const D4: Mask = Mask(0x800000000);
    pub const E4: Mask = Mask(0x1000000000);
    pub const F4: Mask = Mask(0x2000000000);
    pub const G4: Mask = Mask(0x4000000000);
    pub const H4: Mask = Mask(0x8000000000);
    pub const A3: Mask = Mask(0x10000000000);
    pub const B3: Mask = Mask(0x20000000000);
    pub const C3: Mask = Mask(0x40000000000);
    pub const D3: Mask = Mask(0x80000000000);
    pub const E3: Mask = Mask(0x100000000000);
    pub const F3: Mask = Mask(0x200000000000);
    pub const G3: Mask = Mask(0x400000000000);
    pub const H3: Mask = Mask(0x800000000000);
    pub const A2: Mask = Mask(0x1000000000000);
    pub const B2: Mask = Mask(0x2000000000000);
    pub const C2: Mask = Mask(0x4000000000000);
    pub const D2: Mask = Mask(0x8000000000000);
    pub const E2: Mask = Mask(0x10000000000000);
    pub const F2: Mask = Mask(0x20000000000000);
    pub const G2: Mask = Mask(0x40000000000000);
    pub const H2: Mask = Mask(0x80000000000000);
    pub const A1: Mask = Mask(0x100000000000000);
    pub const B1: Mask = Mask(0x200000000000000);
    pub const C1: Mask = Mask(0x400000000000000);
    pub const D1: Mask = Mask(0x800000000000000);
    pub const E1: Mask = Mask(0x1000000000000000);
    pub const F1: Mask = Mask(0x2000000000000000);
    pub const G1: Mask = Mask(0x4000000000000000);
    pub const H1: Mask = Mask(0x8000000000000000);
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

        for file in 0..8 {
            let mut mask = masks::EMPTY;
            let f = File::new(file);
            for rank in 0..8 {
                let sq = Square::from(f, Rank::new(rank));
                mask = mask.with(sq.to_mask());
            }
            println!("pub const {} : Mask = Mask(0x{:X});",
                     f.char().to_ascii_uppercase(),
                     mask.bits());
        }
        println!("");
    }

    #[test]
    fn print_const_ranks() {
        use colored_square::{File, Rank, Square};

        println!("");

        for rank in 0..8 {
            let mut mask = masks::EMPTY;
            let r = Rank::new(rank);
            for file in 0..8 {
                let sq = Square::from(File::new(file), r);
                mask = mask.with(sq.to_mask());
            }
            println!("pub const _{} : Mask = Mask(0x{:X});",
                     r.char(),
                     mask.bits());
        }
        println!("");
    }
    #[test]
    fn print_const_masks() {
        use colored_square::squares;
        println!("");

        for s in squares::All {
            println!("pub const {} : Mask = Mask(0x{:X});",
                     s.to_string().to_uppercase(),
                     s.to_mask().bits());
        }
        println!("");
    }
}
