#![allow(dead_code)]
#![allow(trivial_casts, trivial_numeric_casts)]

use std::fmt::{Display, Result, Formatter};
use nom::IResult;
use nom::IResult::*;

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
impl Castle {
    pub fn parse(input: &str) -> Self {
        parse_castle(input.as_bytes()).unwrap().1
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParsingError {
    Duplication,
    UnrecognizedToken,
}

fn consume(c: char) -> Option<Castle> {
    match c {
        'Q' => Some(WQ),
        'K' => Some(WK),
        'q' => Some(BQ),
        'k' => Some(BK),
        '-' => Some(NONE),
        _ => None,
    }
}

pub fn parse_castle(input: &[u8]) -> IResult<&[u8], Castle, ParsingError> {
    use nom::Err::Position;
    use nom::ErrorKind::Custom;
    use nom::Needed::Unknown;
    use castle::ParsingError::*;

    let mut result = NONE;
    let mut consumed = 0;
    for &e in input {
        match consume(e as char) {
            None => {
                if consumed > 0 {
                    return Done(&input[consumed + 1..], result);
                } else {
                    return Error(Position(Custom(UnrecognizedToken), &input[consumed..]));
                }
            }
            Some(NONE) => return Done(&input[consumed + 1..], result),
            Some(c) => {
                if result.intersects(c) {
                    return Error(Position(Custom(Duplication), &input[consumed..]));
                }
                result |= c;
                consumed += 1;
                if consumed == 4 {
                    return Done(&input[consumed..], result);
                }
            }
        }
    }
    if consumed > 0 {
        Done(&input[consumed..], result)
    } else {
        Incomplete(Unknown)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let check = |sample: &str| {
            let parsed = Castle::parse(sample);
            assert_eq!(format!("{}", parsed), sample)
        };
        check("-");
        check("Qk");
        check("Kq");
        check("Qkq");
        check("Kkq");
        check("KQq");
        check("KQk");
        check("KQkq");
    }
    #[test]
    fn un_canonical() {
        let check = |input: &str, output: &str| {
            let parsed = Castle::parse(input);
            assert_eq!(format!("{}", parsed), output)
        };
        check("kQ", "Qk");
        check("qK", "Kq");
        check("kQq", "Qkq");
        check("qKk", "Kkq");
        check("QKq", "KQq");
        check("QkK", "KQk");
        check("QKqk", "KQkq");
        check("QKqkK", "KQkq");
        check("Q.K", "Q");
    }
    #[test]
    fn duplication() {
        let check = |input: &'static str, expected: usize| {
            use nom::Err::Position;
            use nom::ErrorKind::Custom;
            use castle::ParsingError::*;

            let b = input.as_bytes();
            let err = parse_castle(b).unwrap_err();
            match err {
                Position(Custom(Duplication), reminder) => assert_eq!(reminder, &b[expected..]),
                _ => panic!(err),
            }
        };
        check("kk", 1);
        check("QKkk", 3);
        check("QKkQ", 3);
    }
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
