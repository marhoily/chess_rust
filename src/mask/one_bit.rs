use super::*;

impl Mask {
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
    pub fn has_mote_than_one_bit_set(self) -> bool {
        self.0 & (self.0.wrapping_sub(1)) != 0
    }
    pub fn index_of_least_significant_bit(self) -> u32 {
        self.0.trailing_zeros()
    }
    pub fn index_of_most_significant_bit(self) -> u32 {
        self.0.leading_zeros() ^ 63
    }
    pub fn most_significant_bit(self) -> Mask {
        let mut bb = self.0;
        bb |= bb.wrapping_shr(32);
        bb |= bb.wrapping_shr(16);
        bb |= bb.wrapping_shr(8);
        bb |= bb.wrapping_shr(4);
        bb |= bb.wrapping_shr(2);
        bb |= bb.wrapping_shr(1);
        bb = bb.wrapping_shr(1);
        Mask(bb.wrapping_add(1))
    }
    pub fn least_significant_bit(self) -> Mask {
        let bb = self.0;
        Mask(bb & bb.wrapping_neg())
    }

}

#[cfg(test)]
mod tests {
    use super::super::*;
    use test::Bencher;
    use itertools::*;
    use rand::random;

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
        for (bits, shift) in (0..1000).map(random_bits_shift) {
            let m = Mask((bits | RIGHT).wrapping_shl(shift));
            assert_eq!(m.least_significant_bit().0, RIGHT << shift);
        }
    }
    #[test]
    fn index_of_least_significant_bit() {
        for (bits, shift) in (0..1000).map(random_bits_shift) {
            let m = Mask((bits | RIGHT).wrapping_shl(shift));
            assert_eq!(m.index_of_least_significant_bit(), shift);
        }
    }

    #[test]
    fn most_significant_bit() {
        for (bits, shift) in (0..1000).map(random_bits_shift) {
            let mask = Mask((bits | LEFT).wrapping_shr(shift));
            assert_eq!(mask.most_significant_bit().0, LEFT >> shift);
        }
    }
    #[test]
    fn index_of_most_significant_bit() {
        for (bits, shift) in (0..1000).map(random_bits_shift) {
            let mask = Mask((bits | LEFT).wrapping_shr(shift));
            assert_eq!(mask.index_of_most_significant_bit(), 63 - shift);
        }
    }

    #[test]
    fn single_bits_back_and_forth() {
        for m in (0..1000).map(|_| Mask(random())) {
            assert_equal(m.single_bits().rev(),
                         m.single_bits().collect_vec().into_iter().rev());
        }
    }
    #[test]
    fn single_bit_indices_back_and_forth() {
        for m in (0..1000).map(|_| Mask(random())) {
            assert_equal(m.single_bit_indices().rev(),
                         m.single_bit_indices().collect_vec().into_iter().rev());
        }
    }

    #[bench]
    fn bench_has_mote_than_one_bit_set(b: &mut Bencher) {
        let max: u64 = ::test::black_box(1000);
        b.iter(|| {
            let mut count = 0;
            for n in 0..max {
                if Mask(n).has_mote_than_one_bit_set() {
                    count += 1
                }
            }
            count
        });
    }
    #[bench]
    fn bench_count(b: &mut Bencher) {
        let max: u64 = ::test::black_box(1000);
        b.iter(|| {
            let mut count = 0;
            for n in 0..max {
                if Mask(n).count() > 1 {
                    count += 1
                }
            }
            count
        });
    }

    fn random_bits_shift(_: i32) -> (u64, u32) {
        (random::<u64>(), random::<u32>() % 64)
    }

    const LEFT: u64 = 1u64 << 63;
    const RIGHT: u64 = 1u64;
}