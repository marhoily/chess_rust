use super::*;

impl Mask {
    pub fn single_bits(self) -> MaskIter {
        MaskIter(self)
    }
    pub fn single_bit_indices(self) -> IndexIter {
        IndexIter(self)
    }
}

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct MaskIter(Mask);
impl Iterator for MaskIter {
    type Item = Mask;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == masks::EMPTY {
            None
        } else {
            let mask = self.0;
            let result = mask.least_significant_bit();
            self.0 = Mask(mask.0 & mask.0.wrapping_sub(1));
            Some(result)
        }
    }
}
impl DoubleEndedIterator for MaskIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0 == masks::EMPTY {
            None
        } else {
            let mask = self.0;
            let result = mask.most_significant_bit();
            self.0 = Mask(mask.0 ^ result.0);
            Some(result)
        }
    }
}

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct IndexIter(Mask);
impl Iterator for IndexIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == masks::EMPTY {
            None
        } else {
            let mask = self.0;
            let result = mask.index_of_least_significant_bit();
            self.0 = Mask(mask.0 & mask.0.wrapping_sub(1));
            Some(result)
        }
    }
}
impl DoubleEndedIterator for IndexIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0 == masks::EMPTY {
            None
        } else {
            let mask = self.0;
            let result = mask.index_of_most_significant_bit();
            self.0 = Mask(mask.0 ^ (1u64 << result));
            debug_assert!(mask.count() > self.0.count(),
                          "{:X}, {:X}",
                          mask.0,
                          (self.0).0);
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use quickcheck::*;

    quickcheck! {
        fn count_should_match_single_bits(m : Mask) -> bool {
                m.count() as usize== m.single_bits().count()
        }
        fn count_should_match_single_bit_indices(m : Mask) -> bool {
                m.count() as usize== m.single_bit_indices().count()
        }
    }

    impl Arbitrary for Mask {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Mask(g.next_u64())
        }
    }
}