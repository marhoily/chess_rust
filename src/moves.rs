use kind::*;
use square::*;
use castle::Castle;
use castle;
use std::fmt::{Display, Formatter, Result};

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promote: Kind,
    pub castle: Castle,
}
const CASTLE_Q: Move = Move {
    from: squares::UNDEFINED,
    to: squares::UNDEFINED,
    promote: kinds::UNKNOWN,
    castle: castle::Q,
};
const CASTLE_K: Move = Move {
    from: squares::UNDEFINED,
    to: squares::UNDEFINED,
    promote: kinds::UNKNOWN,
    castle: castle::K,
};

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Move {
            from: from,
            to: to,
            promote: kinds::UNKNOWN,
            castle: castle::NONE,
        }
    }
    pub fn castle(castle: Castle) -> Self {
        Move {
            from: squares::UNDEFINED,
            to: squares::UNDEFINED,
            promote: kinds::UNKNOWN,
            castle: castle,
        }
    }
    pub fn promote(from: Square, to: Square, promote: Kind) -> Self {
        Move {
            from: from,
            to: to,
            promote: promote,
            castle: castle::NONE,
        }
    }

    pub fn parse(input: &str) -> Self {
        parse_move(input.as_bytes()).unwrap().1
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.castle != castle::NONE {
            return write!(f, "{}", if self.castle ==castle::Q {
                "O-O-O"
            } else {
                "O-O"
            });
        }
        try!(write!(f, "{}-{}", self.from, self.to));
        if self.promote != kinds::UNKNOWN {
            try!(write!(f, "={}", self.promote));
        }
        Ok(())
    }
}

named!(parse_promotion(&[u8]) -> Kind,
    complete!(chain!(
        char!('=') ~
        result: alt!(
            value!(kinds::KNIGHT, char!('N')) |
            value!(kinds::BISHOP, char!('B')) |
            value!(kinds::ROOK, char!('R')) |
            value!(kinds::QUEEN, char!('Q')) ),
    || result)));

named!(parse_straight(&[u8]) -> Move,
    chain!(
        from: parse_square ~
        alt!(char!('-') | char!(':')) ? ~
        to: parse_square ~
        promotion: opt!(parse_promotion),
        || Move::promote(from, to, promotion
                .unwrap_or(kinds::UNKNOWN)))
    );

named!(parse_castle(&[u8]) -> Move,
    alt!(
        complete!(value!(CASTLE_Q, tag!("o-o-o"))) |
        complete!(value!(CASTLE_Q, tag!("0-0-0"))) |
        complete!(value!(CASTLE_Q, tag!("O-O-O"))) |
        complete!(value!(CASTLE_K, tag!("o-o"))) |
        complete!(value!(CASTLE_K, tag!("0-0"))) |
        complete!(value!(CASTLE_K, tag!("O-O")))
    ));

named!(pub parse_move(&[u8]) -> Move,
    alt!(parse_straight | parse_castle));


#[cfg(test)]
mod test {
    use square::*;
    use super::*;
    use kind::*;

    #[test]
    fn usual_move() {
        let m = Move::new(squares::E2, squares::E4);
        assert_eq!(format!("{:?}", m),
        "Move { from: Square(52), to: Square(36), promote: Kind(16), castle:  }");
    }

    #[test]
    fn promotion_move() {
        let m = Move::promote(squares::E2, squares::E4, kinds::QUEEN);
        assert_eq!(format!("{:?}", m),
            "Move { from: Square(52), to: Square(36), promote: Kind(4), castle:  }");
    }

    #[test]
    fn parse_usual() {
        assert_eq!(format!("{}", Move::parse("e2e4")), "e2-e4");
        assert_eq!(format!("{}", Move::parse("a1a8")), "a1-a8");
        assert_eq!(format!("{}", Move::parse("c3-f2")), "c3-f2");
        assert_eq!(format!("{}", Move::parse("a8:h1")), "a8-h1");
    }
    #[test]
    fn parse_short_castle() {
        assert_eq!(format!("{}", Move::parse("O-O")), "O-O");
        assert_eq!(format!("{}", Move::parse("0-0")), "O-O");
        assert_eq!(format!("{}", Move::parse("o-o")), "O-O");
    }
    #[test]
    fn parse_long_castle() {
        assert_eq!(format!("{}", Move::parse("O-O-O")), "O-O-O");
        assert_eq!(format!("{}", Move::parse("0-0-0")), "O-O-O");
        assert_eq!(format!("{}", Move::parse("o-o-o")), "O-O-O");
    }

    #[test]
    fn parse_promotion() {
        assert_eq!(format!("{}", Move::parse("e2-e4=Q")), "e2-e4=Q");
    }
}
