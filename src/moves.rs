#![allow(dead_code)]

use kind::*;
use geometry::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promote_to: Kind,
}

impl Move {
    pub fn usual(from: Square, to: Square) -> Self {
        Move {
            from: from,
            to: to,
            promote_to: kinds::UNKNOWN,
        }
    }
    pub fn with_promotion(from: Square, to: Square, promote_to: Kind)-> Self {
        Move {
            from: from,
            to: to,
            promote_to: promote_to,
        }
    }
    pub fn string(self) -> String {
        let mut result = String::with_capacity(6);
        result.push_str(self.from.to_string().as_str());
        result.push('-');
        result.push_str(self.to.to_string().as_str());
        if self.promote_to != kinds::UNKNOWN {
            result.push('=');
            result.push(self.promote_to.char());
        }
        result
    }
    pub fn parse(input: &str) -> Self {
        parse_move(input.as_bytes()).unwrap().1
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
        let e2 = Square::parse("e2");
        let e4 = Square::parse("e4");
        let m = Move::usual(e2, e4);
        assert_eq!(m.from.to_string(), "e2");
        assert_eq!(m.to.to_string(), "e4");
        assert_eq!(m.promote_to, kinds::UNKNOWN);
    }

    #[test]
    fn promotion_move() {
        let e2 = Square::parse("e2");
        let e4 = Square::parse("e4");
        let m = Move::with_promotion(e2, e4, kinds::QUEEN);
        assert_eq!(m.from.to_string(), "e2");
        assert_eq!(m.to.to_string(), "e4");
        assert_eq!(m.promote_to, kinds::QUEEN);
    }

    #[test]
    fn usual_move_to_string() {
        let e2 = Square::parse("e2");
        let e4 = Square::parse("e4");
        assert_eq!(Move::usual(e2, e4).string(), "e2-e4");
    }

    #[test]
    fn promotion_move_to_string() {
        let e2 = Square::parse("e2");
        let e4 = Square::parse("e4");
        assert_eq!(Move::with_promotion(e2, e4, kinds::QUEEN).string(), "e2-e4=Q");
    }

    #[test]
    fn parse_usual_move() {
        assert_eq!(Move::parse("e2e4").string(), "e2-e4");
        assert_eq!(Move::parse("a1a8").string(), "a1-a8");
        assert_eq!(Move::parse("c3-f2").string(), "c3-f2");
        assert_eq!(Move::parse("a8:h1").string(), "a8-h1");
    }

    #[test]
    fn parse_promotion_move() {
        assert_eq!(Move::parse("e2-e4=Q").string(), "e2-e4=Q");
    }
}
