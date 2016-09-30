#![allow(dead_code)]

use piece_types;
use piece_types::*;
use sqares::*;

const MOVE_FROM_MASK: u16 = 0b0000_0000_0000_1111;
const MOVE_TO_MASK: u16 = 0b0000_0000_1111_0000;
const MOVE_PROMOTE_TO_MASK: u16 = 0b0000_0011_0000_0000;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move(u16);

impl Move {
    pub fn usual(from: Square64, to: Square64) -> Self {
        Move::with_promotion(from, to, piece_types::UNKNOWN)
    }
    pub fn with_promotion(from: Square64, to: Square64, promote_to: PieceType) -> Self {
        Move((from.bits() as u16) | ((to.bits() as u16) << 4) | ((promote_to.bits() as u16) << 8))
    }
    pub fn from(self) -> Square64 {
        Square64::new((self.0 & MOVE_FROM_MASK) as u8)
    }
    pub fn to(self) -> Square64 {
        Square64::new(((self.0 & MOVE_TO_MASK) >> 4) as u8)
    }
    pub fn promote_to(self) -> PieceType {
        PieceType::new((((self.0 as u16) & MOVE_PROMOTE_TO_MASK) >> 8) as i32)
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
}


#[cfg(test)]
mod test {
    use sqares::*;
    use super::*;
    use piece_types::*;

    #[test]
    fn usual_move() {
        let e2 = Square64::parse("e2");
        let e4 = Square64::parse("e4");
        let m = Move::usual(e2, e4);
        assert_eq!(m.0, 0);
        assert_eq!(m.from().to_string(), "e2");
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
}
