#![allow(dead_code)]
use std::fmt::{Result, Display, Formatter};
use square88::Square88;
use square88::squares::*;
use piece::Piece;
use piece::pieces::*;

pub struct BitBoard88([Piece; 0x77]);

impl BitBoard88 {
    pub fn new() -> Self {
        BitBoard88([VOID; 0x77])
    }

    pub fn set_piece(&mut self, at: Square88, piece: Piece) {
        self.0[at.bits() as usize] = piece;
    }
    pub fn get_piece(&self, at: Square88) -> Piece {
        self.0[at.bits() as usize]
    }
    pub fn parse(input: &str) -> Self {
        fen::parse_board88(input.as_bytes()).unwrap().1
    }
    pub fn squares(&self) -> SquareIter {
        SquareIter {
            board: self,
            current: FIRST,
        }
    }
}
pub mod fen;

impl Display for BitBoard88 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.print_fen())
    }
}
pub struct SquareIter<'a> {
    board: &'a BitBoard88,
    current: Square88,
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.next().map(|square| self.board.get_piece(square))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use square88::squares::*;
    use piece::pieces::*;

    #[test]
    fn get_piece() {
        let mut b = BitBoard88::new();
        b.set_piece(E2, BLACK_ROOK);
        b.set_piece(E3, BLACK_ROOK);
        assert_eq!(format!("{}", b), "");
    }
}
