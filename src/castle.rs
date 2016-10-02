#![allow(dead_code)]
#![allow(trivial_casts, trivial_numeric_casts)]

use std::fmt::{Display, Result, Formatter};

// todo: make castle be masks for squares that need checking?
// todo: let's keep parser near the struct
bitflags! {
    pub flags Castle: u8 {
        const NONE = 0,
        const Q = WQ.bits | BQ.bits,
        const K = WK.bits | BK.bits,
        const W = WQ.bits | WK.bits,
        const B = BQ.bits | BK.bits,
        const WQ = 1 << 0,
        const WK = 1 << 2,
        const BQ = 1 << 3,
        const BK = 1 << 4,
        const ALL = Q.bits | K.bits,
    }
}

impl Display for Castle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.contains(WK) {
            try!(write!(f, "K"));
        }
        if self.contains(WQ) {
            try!(write!(f, "Q"));
        }
        if self.contains(BK) {
            try!(write!(f, "k"));
        }
        if self.contains(BQ) {
            try!(write!(f, "q"));
        }
        if self.is_empty() {
            try!(write!(f, "-"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{}", ALL), "KQkq");
        assert_eq!(format!("{}", NONE), "-");
        assert_eq!(format!("{}", Q), "Qq");
        assert_eq!(format!("{}", K), "Kk");
        assert_eq!(format!("{}", W), "KQ");
        assert_eq!(format!("{}", B), "kq");
        assert_eq!(format!("{}", WQ), "Q");
        assert_eq!(format!("{}", WK), "K");
        assert_eq!(format!("{}", BQ), "q");
        assert_eq!(format!("{}", BK), "k");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", NONE), "");
        assert_eq!(format!("{:?}", Q), "Q | WQ | BQ");
    }
}