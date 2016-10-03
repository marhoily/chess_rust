use std::fmt::{Result, Display, Formatter};

#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Rank(u8);

impl Rank {
    pub fn new(bits: u8) -> Self {
        debug_assert!(bits < 8);
        Rank(bits)
    }
    pub fn parse(input: char) -> Self {
        debug_assert!((input as u32) < 128, "it is not even an ASCII character!");
        parse_rank(&[input as u8]).unwrap().1
    }
    pub fn char(self) -> char {
        (RANKS[0] - self.0) as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

static RANKS: &'static [u8; 8] = b"87654321";

named!(pub parse_rank(&[u8]) -> Rank,
    map!(is_a!(RANKS), |c:&[u8]| Rank(RANKS[0] - c[0])));

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}

pub mod ranks {
    use super::Rank;

    pub const _1: Rank = Rank(7);
    pub const _2: Rank = Rank(6);
    pub const _3: Rank = Rank(5);
    pub const _4: Rank = Rank(4);
    pub const _5: Rank = Rank(3);
    pub const _6: Rank = Rank(2);
    pub const _7: Rank = Rank(1);
    pub const _8: Rank = Rank(0);

    #[derive(Copy, Clone, Debug)]
    pub struct All;

    impl IntoIterator for All {
        type Item = Rank;
        type IntoIter = Rank;

        fn into_iter(self) -> Self::IntoIter {
            Rank(0)
        }
    }
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rank_char() {
        use super::ranks::*;

        assert_eq!(All.into_iter().
            map(|f| f.char()).collect::<Vec<_>>(),
            ['8', '7', '6', '5', '4', '3', '2', '1']);
    }
    #[test]
    fn rank_display() {
        use super::ranks::*;

        assert_eq!(All.into_iter().
            map(|f| format!("{}", f)).collect::<Vec<_>>(),
            ["8", "7", "6", "5", "4", "3", "2", "1"]);
    }
    #[test]
    fn rank_debug() {
        use super::ranks::*;

        assert_eq!([_1, _8].into_iter().
            map(|f| format!("{:?}", f)).collect::<Vec<_>>(),
            ["Rank(7)", "Rank(0)"]);
    }
    #[test]
    fn rank_parse() {
        use super::ranks::*;

        assert_eq!(['8', '7', '6', '5', '4', '3', '2', '1'].into_iter().
            map(|f| Rank::parse(*f)).collect::<Vec<_>>(),
            [_8, _7, _6, _5, _4, _3, _2, _1]);
    }

}