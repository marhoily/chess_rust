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
        self >> 8
    }
    pub fn shift_south(self) -> Mask {
        self << 8
    }
    pub fn shift_east(self) -> Mask {
        (self << 1) & !masks::files::A
    }
    pub fn shift_north_east(self) -> Mask {
        (self >> 7) & !masks::files::A
    }
    pub fn shift_south_east(self) -> Mask {
        (self << 9) & !masks::files::A
    }
    pub fn shift_west(self) -> Mask {
        (self >> 1) & !masks::files::H
    }
    pub fn shift_north_west(self) -> Mask {
        (self >> 9) & !masks::files::H
    }
    pub fn shift_south_west(self) -> Mask {
        (self << 7) & !masks::files::H
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
        where F: Fn(Mask) -> Mask
    {
        let empty = !stoppers;
        let mut acc = self;
        for _ in 0..7 {
            acc |= empty & shift(acc)
        }
        acc
    }
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
    pub fn has_mote_than_one_bit_set(self) -> bool {
        self.0 & (self.0.wrapping_sub(1)) != 0
    }
    pub fn least_significant_bit(self) -> usize {
        debug_assert!(self.0 != 0);
        LSB[(self.0.wrapping_mul(MAGIC) >> 58) as usize]
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

pub mod masks;

static LSB: &'static [usize] = &[0, 1, 48, 2, 57, 49, 28, 3, 61, 58, 50, 42, 38, 29, 17, 4, 62,
                                 55, 59, 36, 53, 51, 43, 22, 45, 39, 33, 30, 24, 18, 12, 5, 63,
                                 47, 56, 27, 60, 41, 37, 16, 54, 35, 52, 21, 44, 32, 23, 11, 46,
                                 26, 40, 15, 34, 20, 31, 10, 25, 14, 19, 9, 13, 8, 7, 6];

#[allow(dead_code)]
static MSB: &'static [usize] = &[0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54,
                                 58, 35, 52, 50, 42, 21, 44, 38, 32, 29, 23, 17, 11, 4, 62, 46,
                                 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
                                 39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63];

/// The De Bruijn multiplier.
const MAGIC: u64 = 0x03f79d71b4cb0a89;

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn dump() {
        let mask = masks::files::E | masks::ranks::_5 | masks::A8;
        assert_eq!(mask.dump(),
                   "|@^^^@^^^|...\
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
    fn fill() {
        let stoppers = masks::files::B | masks::ranks::_2;
        assert_eq!(stoppers.dump(),
                   "|^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |^@^^^^^^|...\
                    |@@@@@@@@|...\
                    |^@^^^^^^|...");
        assert_eq!(masks::F8.fill(Mask::shift_south_west, stoppers).dump(),
                   "|^^^^^@^^|...\
                    |^^^^@^^^|...\
                    |^^^@^^^^|...\
                    |^^@^^^^^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...");
    }

    #[test]
    fn count() {
        let m = masks::files::B | masks::ranks::_2;
        assert_eq!(m.count(), 15);
    }
    #[test]
    fn has_mote_than_one_bit_set() {
        assert_eq!(masks::files::B.has_mote_than_one_bit_set(), true);
        assert_eq!(masks::B2.has_mote_than_one_bit_set(), false);
        assert_eq!(masks::EMPTY.has_mote_than_one_bit_set(), false);
    }
    #[test]
    fn least_significant_bit() {
        let x: u64 = ::rand::random();
        let x = x | 1;
        let shift = ::rand::random::<usize>() % 64;
        let x = x << shift;
        let x = Mask(x);
        assert_eq!(x.least_significant_bit(), shift);
    }
    #[bench]
    fn bench_has_mote_than_one_bit_set(b: &mut Bencher) {
        b.iter(|| masks::files::B.has_mote_than_one_bit_set());
    }
    #[bench]
    fn bench_count(b: &mut Bencher) {
        b.iter(|| masks::files::B.count() > 1);
    }
}
