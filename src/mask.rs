use geometry::Square;
use std::ops::{BitOr, BitOrAssign, BitAnd, BitAndAssign, Shl, ShlAssign, Shr, ShrAssign, Not};

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Mask(u64);

impl Mask {
    pub fn new(bits: u64) -> Self {
        Mask(bits)
    }
    pub fn from(square: Square) -> Self {
        Mask(1 << square.bits())
    }
    pub fn bits(self) -> u64 {
        self.0
    }
    pub fn empty(self) -> bool {
        self.0 == 0
    }
    pub fn has_all(self, another: Mask) -> bool {
        self.0 & another.bits() == another.bits()
    }
    pub fn has_any(self, another: Mask) -> bool {
        self.0 & another.bits() != 0
    }
    pub fn shift_north(self) -> Mask {
        self << 8
    }
    pub fn shift_south(self) -> Mask {
        self >> 8
    }
    pub fn shift_east(self) -> Mask {
        (self << 1) & !masks::files::A
    }
    pub fn shift_north_east(self) -> Mask {
        (self << 9) & !masks::files::A
    }
    pub fn shift_south_east(self) -> Mask {
        (self >> 7) & !masks::files::A
    }
    pub fn shift_west(self) -> Mask {
        (self >> 1) & !masks::files::H
    }
    pub fn shift_north_west(self) -> Mask {
        (self << 7) & !masks::files::H
    }
    pub fn shift_south_west(self) -> Mask {
        (self >> 9) & !masks::files::H
    }
    pub fn dump(self) -> String {
        let mut result = String::with_capacity(100);
        result.push('|');
        for rank in masks::ranks::ALL {
            for file in masks::files::ALL {
                if self.has_any(*file & *rank) {
                    result.push('@');
                } else {
                    result.push('^');
                }
            }
            result.push('|');
            result.push('.');
            result.push('.');
            result.push('.');
            result.push('|');
        }
        result.pop();
        result
    }
    pub fn flip_horizontally(self) -> Mask {
        let x = Mask(0x5555555555555555);
        let y = Mask(0x3333333333333333);
        let z = Mask(0x0F0F0F0F0F0F0F0F);
        let mut n = self;
        n = ((n >> 1) & x) | ((n & x) << 1);
        n = ((n >> 2) & y) | ((n & y) << 2);
        n = ((n >> 4) & z) | ((n & z) << 4);
        n
    }
    pub fn flip_vertically(self) -> Mask {
        let x = Mask(0x00FF00FF00FF00FF);
        let y = Mask(0x0000FFFF0000FFFF);
        let mut n = self;
        n = ((n >> 8) & x) | ((n & x) << 8);
        n = ((n >> 16) & y) | ((n & y) << 16);
        n = (n >> 32) | (n << 32);
        n
    }
    pub fn fill<F>(self, shift: F, stoppers: Mask) -> Mask
        where F: Fn(Mask) -> Mask {
        let empty = !stoppers;
        let mut acc = self;
        for _ in 0..7 {
            acc |= empty & shift(acc)
        }
        acc
    }
}
impl BitOr<Mask> for Mask {
    type Output = Mask;
    fn bitor(self, rhs: Mask) -> Self::Output {
        Mask(self.0 | rhs.0)
    }
}
impl BitOrAssign<Mask> for Mask {
    fn bitor_assign(&mut self, rhs: Mask) {
        self.0 |= rhs.0
    }
}
impl BitAnd<Mask> for Mask {
    type Output = Mask;
    fn bitand(self, rhs: Mask) -> Self::Output {
        Mask(self.0 & rhs.0)
    }
}
impl BitAndAssign<Mask> for Mask {
    fn bitand_assign(&mut self, rhs: Mask) {
        self.0 &= rhs.0
    }
}
impl Shl<u8> for Mask {
    type Output = Mask;
    fn shl(self, rhs: u8) -> Self::Output {
        Mask(self.0 << rhs)
    }
}
impl ShlAssign<u8> for Mask {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}
impl Shr<u8> for Mask {
    type Output = Mask;
    fn shr(self, rhs: u8) -> Self::Output {
        Mask(self.0 >> rhs)
    }
}
impl ShrAssign<u8> for Mask {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}
impl Not for Mask {
    type Output = Mask;
    fn not(self) -> Self::Output {
        Mask(!self.0)
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

        pub const _1: Mask = Mask(0xFF);
        pub const _2: Mask = Mask(0xFF00);
        pub const _3: Mask = Mask(0xFF0000);
        pub const _4: Mask = Mask(0xFF000000);
        pub const _5: Mask = Mask(0xFF00000000);
        pub const _6: Mask = Mask(0xFF0000000000);
        pub const _7: Mask = Mask(0xFF000000000000);
        pub const _8: Mask = Mask(0xFF00000000000000);

        pub static ALL: &'static [Mask] = &[_8, _7, _6, _5, _4, _3, _2, _1];
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

    #[derive(Copy, Clone, Debug)]
    pub struct All;

    impl IntoIterator for All {
        type Item = Mask;
        type IntoIter = MaskIter;

        fn into_iter(self) -> Self::IntoIter {
            MaskIter::new()
        }
    }

    #[derive(Default, Copy, Clone, Debug)]
    pub struct MaskIter(u64);

    impl MaskIter {
        pub fn new() -> Self {
            MaskIter(1)
        }
    }

    impl Iterator for MaskIter {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::*;

    #[test]
    fn all_squares_exp() {
        let all = masks::All.into_iter()
            .collect::<Vec<_>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], Mask(1));
        assert_eq!(all[63], Mask(1 << 63));
    }

    #[test]
    fn dump() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.dump(),
                   "|^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |@@@@@@@@|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...");
    }
    #[test]
    fn shift_north() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_north().dump(),
                   "|^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |@@@@@@@@|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^^^^^|...");
    }
    #[test]
    fn shift_south() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_south().dump(),
                   "|^^^^^^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |@@@@@@@@|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...");
    }
    #[test]
    fn shift_east() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_east().dump(),
                   "|^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^@@@@@@@|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...");
    }
    #[test]
    fn shift_south_east() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_south_east().dump(),
                   "|^^^^^^^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^@@@@@@@|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...");
    }
    #[test]
    fn shift_north_east() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_north_east().dump(),
                   "|^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^@@@@@@@|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^^^^|...");
    }
    #[test]
    fn shift_west() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_west().dump(),
                   "|^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |@@@@@@@^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...");
    }
    #[test]
    fn shift_south_west() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_south_west().dump(),
                   "|^^^^^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |@@@@@@@^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...");
    }
    #[test]
    fn shift_north_west() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_north_west().dump(),
                   "|^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |@@@@@@@^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^^^^^^|...");

    }

    #[test]
    fn flip_horizontally() {
        let mask = masks::files::B | masks::ranks::_2;
        assert_eq!(mask.dump(),
                   "|^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |@@@@@@@@|...\
                    |^@^^^^^^|...");
        assert_eq!(mask.flip_horizontally().dump(),
                   "|^^^^^^@^|...\
                    |^^^^^^@^|...\
                    |^^^^^^@^|...\
                    |^^^^^^@^|...\
                    |^^^^^^@^|...\
                    |^^^^^^@^|...\
                    |@@@@@@@@|...\
                    |^^^^^^@^|...");
    }

    #[test]
    fn flip_vertically() {
        let mask = masks::files::B | masks::ranks::_2;
        assert_eq!(mask.dump(),
                   "|^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |@@@@@@@@|...\
                    |^@^^^^^^|...");
        assert_eq!(mask.flip_vertically().dump(),
                   "|^@^^^^^^|...\
                    |@@@@@@@@|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...");
    }


    #[test]
    fn print_const_files() {
        use geometry::{File, Rank, Square};
        use std::ascii::AsciiExt;

        println!("");

        for file in 0..8 {
            let mut mask = masks::EMPTY;
            let f = File::new(file);
            for rank in 0..8 {
                let sq = Square::from(f, Rank::new(rank));
                mask |= sq.mask();
            }
            println!("pub const {} : Mask = Mask(0x{:X});",
                     f.char().to_ascii_uppercase(),
                     mask.bits());
        }
        println!("");
    }
    #[test]
    fn print_const_ranks() {
        use geometry::{File, Rank, Square};

        println!("");

        for rank in 0..8 {
            let mut mask = masks::EMPTY;
            let r = Rank::new(rank);
            for file in 0..8 {
                let sq = Square::from(File::new(file), r);
                mask |= sq.mask();
            }
            println!("pub const _{} : Mask = Mask(0x{:X});",
                     r.char(),
                     mask.bits());
        }
        println!("");
    }
    #[test]
    fn print_const_masks() {
        use geometry::squares;
        println!("");

        for s in squares::All {
            println!("pub const {} : Mask = Mask(0x{:X});",
                     s.to_string().to_uppercase(),
                     s.mask().bits());
        }
        println!("");
    }
}
