use std::fmt::{Result, Display, Formatter};

#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Rank(u8);

impl Rank {
    pub fn from_bits(bits: u8) -> Self {
        debug_assert!(bits < 8);
        Rank(bits)
    }
    pub fn parse(input: char) -> Self {
        debug_assert!((input as u32) < 128, "it is not even an ASCII character!");
        parse_rank(&[input as u8]).unwrap().1
    }
    pub fn char(self) -> char {
        RANK_SYMBOLS[self.0 as usize] as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

static RANK_SYMBOLS: &'static [u8; 8] = b"87654321";

named!(pub parse_rank(&[u8]) -> Rank,
    map!(is_a!(RANK_SYMBOLS), |c:&[u8]| Rank(RANK_SYMBOLS[0] - c[0])));

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}

pub const ALL_RANKS: Rank = Rank(0);

pub const _1: Rank = Rank(7);
pub const _2: Rank = Rank(6);
pub const _3: Rank = Rank(5);
pub const _4: Rank = Rank(4);
pub const _5: Rank = Rank(3);
pub const _6: Rank = Rank(2);
pub const _7: Rank = Rank(1);
pub const _8: Rank = Rank(0);

impl Iterator for Rank {
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 8 {
            None
        } else {
            let result = *self;
            self.0 += 1;
            Some(result)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::*;

    #[test]
    fn all_ranks() {
        assert_eq!(ALL_RANKS.collect_vec(),
            [_8, _7, _6, _5, _4, _3, _2, _1]);
    }

    #[test]
    fn rank_char() {
        assert_eq!(ALL_RANKS.
            map(|f| f.char()).collect::<Vec<_>>(),
            ['8', '7', '6', '5', '4', '3', '2', '1']);
    }
    #[test]
    fn rank_display() {
        assert_eq!(ALL_RANKS.
            map(|f| format!("{}", f)).collect::<Vec<_>>(),
            ["8", "7", "6", "5", "4", "3", "2", "1"]);
    }
    #[test]
    fn rank_debug() {
        assert_eq!([_1, _8].into_iter().
            map(|f| format!("{:?}", f)).collect::<Vec<_>>(),
            ["Rank(7)", "Rank(0)"]);
    }
    #[test]
    fn rank_parse() {
        assert_eq!(['8', '7', '6', '5', '4', '3', '2', '1'].into_iter().
            map(|f| Rank::parse(*f)).collect::<Vec<_>>(),
            [_8, _7, _6, _5, _4, _3, _2, _1]);
    }
}