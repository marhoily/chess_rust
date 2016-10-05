use std::fmt::{Result, Display, Formatter};
use piece::Piece;
use piece::pieces::*;
use mask::Mask;
use mask::masks::*;

#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct BitBoard88([Mask; COUNT]);

impl BitBoard88 {
    pub fn new() -> Self {
        BitBoard88([EMPTY; COUNT])
    }
    fn index(&self, piece: Piece) -> Mask {
        self.0[piece.bits() as usize]
    }
    pub fn white_pawns(&self) -> Mask {
        self.index(WHITE_PAWN)
    }
    pub fn white_knights(&self) -> Mask {
        self.index(WHITE_KNIGHT)
    }
    pub fn white_bishops(&self) -> Mask {
        self.index(WHITE_BISHOP)
    }
    pub fn white_rooks(&self) -> Mask {
        self.index(WHITE_ROOK)
    }
    pub fn white_queens(&self) -> Mask {
        self.index(WHITE_QUEEN)
    }
    pub fn white_kings(&self) -> Mask {
        self.index(WHITE_KING)
    }
    pub fn black_pawns(&self) -> Mask {
        self.index(BLACK_PAWN)
    }
    pub fn black_knights(&self) -> Mask {
        self.index(BLACK_KNIGHT)
    }
    pub fn black_bishops(&self) -> Mask {
        self.index(BLACK_BISHOP)
    }
    pub fn black_rooks(&self) -> Mask {
        self.index(BLACK_ROOK)
    }
    pub fn black_queens(&self) -> Mask {
        self.index(BLACK_QUEEN)
    }
    pub fn black_kings(&self) -> Mask {
        self.index(BLACK_KING)
    }

    pub fn set_piece(&mut self, square: Mask, piece: Piece) {
        let idx = piece.bits() as usize;
        self.0[idx] |= square;
    }
    pub fn get_piece(&self, square: Mask) -> Piece {
        for probe in Pieces {
            if self.index(probe).has_any(square) {
                return probe;
            }
        }
        VOID
    }
    pub fn squares(&self) -> SquareIter {
        SquareIter {
            board: self,
            current: MaskIter::new(),
        }
    }
    pub fn parse(input: &str) -> Self {
        fen::parse_bit_board88(input.as_bytes()).unwrap().1
    }
    pub fn occupation(&self) -> Mask {
        self.0.iter().fold(EMPTY, |acc, &x| acc | x)
    }
    pub fn white_occupation(&self) -> Mask {
        self.0[..6].iter().fold(EMPTY, |acc, &x| acc | x)
    }
    pub fn black_occupation(&self) -> Mask {
        self.0[6..].iter().fold(EMPTY, |acc, &x| acc | x)
    }
    pub fn white_attacks(&self) -> Mask {
        let stoppers = self.black_occupation();
        let a = self.white_pawns().white_pawn_attacks();
        let b = self.white_knights().knight_attacks();
        let c = self.white_bishops().bishop_attacks(stoppers);
        let d = self.white_rooks().rook_attacks(stoppers);
        let e = self.white_queens().queen_attacks(stoppers);
        let f = self.white_kings().king_attacks();
        a | b | c | d | e | f
    }
}
pub mod fen;

impl Display for BitBoard88 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.print_fen())
    }
}
#[derive(Copy, Clone, Debug)]
pub struct SquareIter<'a> {
    board: &'a BitBoard88,
    current: MaskIter,
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
    use std::iter::*;
    use piece::pieces::*;
    use mask::masks::*;

    #[test]
    fn get_piece() {
        let mut b = BitBoard88::new();
        b.set_piece(E2, BLACK_ROOK);
        b.set_piece(E3, BLACK_ROOK);
        assert_eq!(b.get_piece(E2), BLACK_ROOK);
        assert_eq!(b.get_piece(E3), BLACK_ROOK);
    }

    #[test]
    fn bit_board88_squares() {
        let mut b = BitBoard88::new();
        b.set_piece(A8, BLACK_ROOK);

        let all = b.squares().collect::<Vec<_>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], BLACK_ROOK);
        assert_eq!(all[63], VOID);
    }

    #[test]
    fn occupation() {
        assert_eq!(sample_with_one_of_each_kind().occupation().dump(),
                   "|@@@@@@^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|..\
                    .|^^^^^^^^|...|@@@@@@^^|...");
    }
    #[test]
    fn black_occupation() {
        assert_eq!(sample_with_one_of_each_kind().black_occupation().dump(),
                   "|@@@@@@^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|..\
                    .|^^^^^^^^|...|^^^^^^^^|...");
    }
    #[test]
    fn white_occupation() {
        assert_eq!(sample_with_one_of_each_kind().white_occupation().dump(),
                   "|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|...|^^^^^^^^|..\
                    .|^^^^^^^^|...|@@@@@@^^|...");
    }
    fn sample_with_one_of_each_kind() -> BitBoard88 {
        let mut b = BitBoard88::new();
        b.set_piece(A8, BLACK_ROOK);
        b.set_piece(B8, BLACK_BISHOP);
        b.set_piece(C8, BLACK_KING);
        b.set_piece(D8, BLACK_QUEEN);
        b.set_piece(E8, BLACK_PAWN);
        b.set_piece(F8, BLACK_KNIGHT);

        b.set_piece(A1, WHITE_ROOK);
        b.set_piece(B1, WHITE_BISHOP);
        b.set_piece(C1, WHITE_KING);
        b.set_piece(D1, WHITE_QUEEN);
        b.set_piece(E1, WHITE_PAWN);
        b.set_piece(F1, WHITE_KNIGHT);
        b
    }
}
