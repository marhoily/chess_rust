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

    #[test]
    fn c6_is_attacked_by_black_pawn_on_b7() {
        assert_is_attacked("8/1p6/8/8/8/8/8/8 w", C6/*Black*/);
    }

    #[test]
    fn a6_is_attacked_by_black_pawn_on_b7() {
        assert_is_attacked("8/1p6/8/8/8/8/8/8 w", A6/*Black*/);
    }

    #[test]
    fn when_check_if_a8_is_attacked_it_does_not_overflow() {
        assert_is_attacked("8/8/8/8/8/8/8/8 w", A8/*Black*//*_not*/);
    }

    #[test]
    fn h8_is_attacked_by_black_bishop_on_d4() {
        assert_is_attacked("8/8/8/8/3b4/8/8/8 w", H8/*Black*/);
    }

    #[test]
    fn h8_is_NOT_attacked_by_black_bishop_on_d4_because_its_masked_by_the_pawn_on_f6() {
        assert_is_attacked("8/8/5P2/8/3b4/8/8/8 w", H8/*Black*//*_not*/);
    }

    #[test]
    fn a7_is_attacked_by_black_bishop_on_d4() {
        assert_is_attacked("8/8/8/8/3b4/8/8/8 w", A7/*Black*/);
    }

    #[test]
    fn a1_is_attacked_by_black_bishop_on_d4() {
        assert_is_attacked("8/8/8/8/3b4/8/8/8 w", A1/*Black*/);
    }

    #[test]
    fn f2_is_attacked_by_black_bishop_on_d4() {
        assert_is_attacked("8/8/8/8/3b4/8/8/8 w", F2/*Black*/);
    }

    #[test]
    fn c2_is_attacked_by_black_knight_on_d4() {
        assert_is_attacked("8/8/8/8/3n4/8/8/8 w", C2/*Black*/);
    }

    #[test]
    fn b3_is_attacked_by_black_knight_on_d4() {
        assert_is_attacked("8/8/8/8/3n4/8/8/8 w", B3/*Black*/);
    }

    #[test]
    fn b5_is_attacked_by_black_knight_on_d4() {
        assert_is_attacked("8/8/8/8/3n4/8/8/8 w", B5/*Black*/);
    }

    #[test]
    fn c6_is_attacked_by_black_knight_on_d4() {
        assert_is_attacked("8/8/8/8/3n4/8/8/8 w", C6/*Black*/);
    }

    #[test]
    fn e6_is_attacked_by_black_knight_on_d4() {
        assert_is_attacked("8/8/8/8/3n4/8/8/8 w", E6/*Black*/);
    }

    #[test]
    fn f5_is_attacked_by_black_knight_on_d4() {
        assert_is_attacked("8/8/8/8/3n4/8/8/8 w", F5/*Black*/);
    }

    #[test]
    fn f3_is_attacked_by_black_knight_on_d4() {
        assert_is_attacked("8/8/8/8/3n4/8/8/8 w", f3/*Black*/);
    }

    #[test]
    fn e2_is_attacked_by_black_knight_on_d4() {
        assert_is_attacked("8/8/8/8/3n4/8/8/8 w", e2/*Black*/);
    }

    #[test]
    fn d1_is_attacked_by_black_rook_on_d4() {
        assert_is_attacked("8/8/8/8/3r4/8/8/8 w", d1/*Black*/);
    }

    #[test]
    fn d6_is_attacked_by_black_rook_on_d4() {
        assert_is_attacked("8/8/8/8/3r4/8/8/8 w", d6/*Black*/);
    }

    #[test]
    fn f4_is_attacked_by_black_rook_on_d4() {
        assert_is_attacked("8/8/8/8/3r4/8/8/8 w", f4/*Black*/);
    }

    #[test]
    fn a4_is_attacked_by_black_rook_on_d4() {
        assert_is_attacked("8/8/8/8/3r4/8/8/8 w", a4/*Black*/);
    }

    #[test]
    fn c4_is_attacked_by_black_queen_on_d4() {
        assert_is_attacked("8/8/8/8/3q4/8/8/8 w", c4/*Black*/);
    }

    #[test]
    fn c3_is_attacked_by_black_queen_on_d4() {
        assert_is_attacked("8/8/8/8/3q4/8/8/8 w", c3/*Black*/);
    }

    #[test]
    fn d3_is_attacked_by_black_queen_on_d4() {
        assert_is_attacked("8/8/8/8/3q4/8/8/8 w", d3/*Black*/);
    }

    #[test]
    fn e3_is_attacked_by_black_queen_on_d4() {
        assert_is_attacked("8/8/8/8/3q4/8/8/8 w", e3/*Black*/);
    }

    #[test]
    fn e4_is_attacked_by_black_queen_on_d4() {
        assert_is_attacked("8/8/8/8/3q4/8/8/8 w", e4/*Black*/);
    }

    #[test]
    fn e5_is_attacked_by_black_queen_on_d4() {
        assert_is_attacked("8/8/8/8/3q4/8/8/8 w", e5/*Black*/);
    }

    #[test]
    fn d5_is_attacked_by_black_queen_on_d4() {
        assert_is_attacked("8/8/8/8/3q4/8/8/8 w", d5/*Black*/);
    }

    #[test]
    fn c5_is_attacked_by_black_queen_on_d4() {
        assert_is_attacked("8/8/8/8/3q4/8/8/8 w", c5/*Black*/);
    }
    // =============== King ===============
    #[test]
    fn c4_is_attacked_by_black_king_on_d4() {
        assert_is_attacked("8/8/8/8/3k4/8/8/8 w", c4/*Black*/);
    }

    #[test]
    fn c3_is_attacked_by_black_king_on_d4() {
        assert_is_attacked("8/8/8/8/3k4/8/8/8 w", c3/*Black*/);
    }

    #[test]
    fn d3_is_attacked_by_black_king_on_d4() {
        assert_is_attacked("8/8/8/8/3k4/8/8/8 w", d3/*Black*/);
    }

    #[test]
    fn e3_is_attacked_by_black_king_on_d4() {
        assert_is_attacked("8/8/8/8/3k4/8/8/8 w", e3/*Black*/);
    }

    #[test]
    fn e4_is_attacked_by_black_king_on_d4() {
        assert_is_attacked("8/8/8/8/3k4/8/8/8 w", e4/*Black*/);
    }

    #[test]
    fn e5_is_attacked_by_black_king_on_d4() {
        assert_is_attacked("8/8/8/8/3k4/8/8/8 w", e5/*Black*/);
    }

    #[test]
    fn d5_is_attacked_by_black_king_on_d4() {
        assert_is_attacked("8/8/8/8/3k4/8/8/8 w", d5/*Black*/);
    }

    #[test]
    fn c5_is_attacked_by_black_king_on_d4() {
        assert_is_attacked("8/8/8/8/3k4/8/8/8 w", c5/*Black*/);
    }
    // =============== Color.White ==============
    #[test]
    fn c8_is_attacked_by_white_pawn_on_b7() {
        assert_is_attacked("8/1P6/8/8/8/8/8/8 w", c8/*White*/);
    }

    #[test]
    fn a8_is_attacked_by_white_pawn_on_b7() {
        assert_is_attacked("8/1P6/8/8/8/8/8/8 w", a8/*White*/);
    }
}

fn assert_is_attacked(fen: &str, square: Square88) {
    assert_eq!(BitBoard88::parse(fen).is_attacked_by(square), true);
}

fn assert_is_not_attacked(fen: &str, square: Square88) {
    assert_eq!(BitBoard88::parse(fen).is_attacked_by(square), false);
}
}
