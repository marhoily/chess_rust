#![allow(dead_code)]

use std::fmt::{Result, Display, Formatter};
use square88::Square88;
use square88::squares::*;
use piece::Piece;
use piece::pieces::*;

pub struct BitBoard88([Piece; 0x78]);

impl BitBoard88 {
    pub fn new() -> Self {
        BitBoard88([VOID; 0x78])
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
    pub fn is_attacked_by_jump(&self, square: Square88, piece: Piece, increments: &[i8]) -> bool {
        increments.iter().map(|&i| {
            let target = square + i;
            target.is_valid() && self.get_piece(target) == piece
        }).any(|x| x)
    }
    pub fn is_attacked_black_pawn(&self, square: Square88) -> bool {
        self.is_attacked_by_jump(square, BLACK_PAWN, &[-15, -17])
    }
    pub fn is_attacked_by(&self, square: Square88) -> bool {
        self.is_attacked_black_pawn(square)
    }

    /// slide from a `square` in direction of `increment` looking for a `piece`.
    /// return the index if found, invalid square otherwise
    #[allow(if_same_then_else)]
    pub fn slide_for(&self, square: Square88, piece: Piece, increment: i8) -> Square88 {
        let next = square + increment;
        if !next.is_valid() { return INVALID }
        if self.get_piece(next) == piece { return next }
        if self.get_piece(next) != VOID { return INVALID }
        self.slide_for(next, piece, increment)
    }
    pub fn find(&self, piece: Piece) -> Square88 {
        for s in ::square88::squares::All {
            if self.get_piece(s) == piece {
                return s
            }
        }
        INVALID
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
    use square88::Square88;
    use square88::squares::*;
    use piece::pieces::*;

    #[test]
    fn set_piece() {
        let mut b = BitBoard88::new();
        b.set_piece(E2, BLACK_ROOK);
        b.set_piece(E3, BLACK_ROOK);
        assert_eq!(format!("{}", b), "8/8/8/8/8/4r3/4r3/8");
    }

    #[test]
    fn get_piece() {
        let b = BitBoard88::parse("8/8/8/8/8/4r3/4r3/8");
        assert_eq!(b.get_piece(E2), BLACK_ROOK);
        assert_eq!(b.get_piece(E3), BLACK_ROOK);
    }

    #[test]
    fn c6_is_attacked_by_black_pawn_on_b7() {
        assert_is_attacked("8/1p6/8/8/8/8/8/8", C6);
    }
    fn assert_is_attacked(fen: &str, square: Square88) {
        assert_eq!(BitBoard88::parse(fen).is_attacked_by(square), true);
    }
    fn assert_is_not_attacked(fen: &str, square: Square88) {
        assert_eq!(BitBoard88::parse(fen).is_attacked_by(square), false);
    }

}
