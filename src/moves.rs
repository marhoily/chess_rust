#![allow(dead_code)]

use piece_types;
use piece_types::*;
use colored_squares::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move(u32);

impl Move {
    pub fn usual(from: Square64, to: Square64) -> Self {
        Move::with_promotion(from, to, piece_types::UNKNOWN)
    }
    pub fn with_promotion(from: Square64, to: Square64, promote_to: PieceType) -> Self {
        let mut bits: u32 = 0;
        bits |= promote_to.bits() as u32;
        bits <<= 8;
        bits |= to.bits() as u32;
        bits <<= 8;
        bits |= from.bits() as u32;
        Move(bits)
    }
    pub fn from(self) -> Square64 {
        Square64::new(self.0 as u8)
    }
    pub fn to(self) -> Square64 {
        Square64::new((self.0 >> 8) as u8)
    }
    pub fn promote_to(self) -> PieceType {
        PieceType::new((self.0 >> 16) as u8)
    }
    pub fn string(self) -> String {
        let mut result = String::with_capacity(6);
        result.push_str(self.from().to_string().as_str());
        result.push('-');
        result.push_str(self.to().to_string().as_str());
        let promote_to = self.promote_to();
        if promote_to != piece_types::UNKNOWN {
            result.push('=');
            result.push(promote_to.char());
        }
        result
    }
    pub fn parse(input: &str) -> Self {
        parse_move(input.as_bytes()).unwrap().1
    }
}

named!(parse_promotion(&[u8]) -> PieceType,
    complete!(chain!(
        char!('=') ~
        result: alt!(
            value!(KNIGHT, char!('N')) |
            value!(BISHOP, char!('B')) |
            value!(ROOK, char!('R')) |
            value!(QUEEN, char!('Q')) ),
    || result)));

named!(pub parse_move(&[u8]) -> Move,
    chain!(
        from: parse_square ~
        alt!(char!('-') | char!(':')) ? ~
        to: parse_square ~
        promotion: opt!(parse_promotion),
        || Move::with_promotion(from, to, promotion.unwrap_or(UNKNOWN)))
    );


#[cfg(test)]
mod test {
    use colored_squares::*;
    use super::*;
    use piece_types::*;

    #[test]
    fn usual_move() {
        let e2 = Square64::parse("e2");
        let e4 = Square64::parse("e4");
        let m = Move::usual(e2, e4);
        assert_eq!(m.from().to_string(), "e2");
        assert_eq!(m.to().to_string(), "e4");
        assert_eq!(m.promote_to(), UNKNOWN);
    }

    #[test]
    fn promotion_move() {
        let e2 = Square64::parse("e2");
        let e4 = Square64::parse("e4");
        let m = Move::with_promotion(e2, e4, QUEEN);
        assert_eq!(m.from().to_string(), "e2");
        assert_eq!(m.to().to_string(), "e4");
        assert_eq!(m.promote_to(), QUEEN);
    }

    #[test]
    fn usual_move_to_string() {
        let e2 = Square64::parse("e2");
        let e4 = Square64::parse("e4");
        assert_eq!(Move::usual(e2, e4).string(), "e2-e4");
    }

    #[test]
    fn promotion_move_to_string() {
        let e2 = Square64::parse("e2");
        let e4 = Square64::parse("e4");
        assert_eq!(Move::with_promotion(e2, e4, QUEEN).string(), "e2-e4=Q");
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
