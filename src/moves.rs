#![allow(dead_code)]

use kind::*;
use geometry::*;
use std::fmt::{Display, Formatter, Result};

// todo: separate representation for castling moves
#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promote_to: Kind,
}

impl Move {
    pub fn usual(from: Square, to: Square) -> Self {
        Self::with_promotion(from, to, kinds::UNKNOWN)
    }
    pub fn with_promotion(from: Square, to: Square, promote_to: Kind) -> Self {
        Move {
            from: from,
            to: to,
            promote_to: promote_to,
        }
    }

    pub fn parse(input: &str) -> Self {
        parse_move(input.as_bytes()).unwrap().1
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        try!(write!(f, "{}-{}", self.from, self.to));
        if self.promote_to != kinds::UNKNOWN {
            try!(write!(f, "={}", self.promote_to));
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

named!(pub parse_move(&[u8]) -> Move,
    chain!(
        from: parse_square ~
        alt!(char!('-') | char!(':')) ? ~
        to: parse_square ~
        promotion: opt!(parse_promotion),
        || Move::with_promotion(from, to, promotion
                .unwrap_or(kinds::UNKNOWN)))
    );


#[cfg(test)]
mod test {
    use geometry::*;
    use super::*;
    use kind::*;

    #[test]
    fn usual_move() {
        let m = Move::usual(squares::E2, squares::E4);
        assert_eq!(format!("{:?}", m),
        "Move { from: Square(52), to: Square(36), promote_to: Kind(16) }");
    }

    #[test]
    fn promotion_move() {
        let m = Move::with_promotion(
            squares::E2, squares::E4, kinds::QUEEN);
        assert_eq!(format!("{:?}", m),
            "Move { from: Square(52), to: Square(36), promote_to: Kind(4) }");
    }

    #[test]
    fn parse_usual() {
        assert_eq!(format!("{}", Move::parse("e2e4")), "e2-e4");
        assert_eq!(format!("{}", Move::parse("a1a8")), "a1-a8");
        assert_eq!(format!("{}", Move::parse("c3-f2")), "c3-f2");
        assert_eq!(format!("{}", Move::parse("a8:h1")), "a8-h1");
    }

    #[test]
    fn parse_promotion() {
        assert_eq!(format!("{}", Move::parse("e2-e4=Q")), "e2-e4=Q");
    }
}
