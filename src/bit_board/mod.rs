use std::fmt::{Result, Display, Formatter};
use piece::Piece;
use piece::pieces::*;
use mask::Mask;
use mask::masks::*;

#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct BitBoard([Mask; COUNT]);

impl BitBoard {
    pub fn new() -> Self {
        BitBoard([EMPTY; COUNT])
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
        fen::parse_bit_board(input.as_bytes()).unwrap().1
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
        let stoppers = self.occupation();
        let a = self.white_pawns().white_pawn_attacks();
        let b = self.white_knights().knight_attacks();
        let c = self.white_bishops().bishop_attacks(stoppers);
        let d = self.white_rooks().rook_attacks(stoppers);
        let e = self.white_queens().queen_attacks(stoppers);
        let f = self.white_kings().king_attacks();
        a | b | c | d | e | f
    }
    pub fn black_attacks(&self) -> Mask {
        let stoppers = self.occupation();
        let a = self.black_pawns().black_pawn_attacks();
        let b = self.black_knights().knight_attacks();
        let c = self.black_bishops().bishop_attacks(stoppers);
        let d = self.black_rooks().rook_attacks(stoppers);
        let e = self.black_queens().queen_attacks(stoppers);
        let f = self.black_kings().king_attacks();
        a | b | c | d | e | f
    }
    pub fn swap_colors(&mut self) {
        for i in 0..6 {
            self.0[..].swap(i, i + 6)
        }
    }
}

pub mod fen;

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.print_fen())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SquareIter<'a> {
    board: &'a BitBoard,
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
    use rand::*;
    use mask::Mask;
    use board88::BitBoard88;

    #[test]
    fn get_piece() {
        let mut b = BitBoard::new();
        b.set_piece(E2, BLACK_ROOK);
        b.set_piece(E3, BLACK_ROOK);
        assert_eq!(b.get_piece(E2), BLACK_ROOK);
        assert_eq!(b.get_piece(E3), BLACK_ROOK);
    }

    #[test]
    fn bit_board_squares() {
        let mut b = BitBoard::new();
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

    fn sample_with_one_of_each_kind() -> BitBoard {
        let mut b = BitBoard::new();
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

    #[test]
    fn white_attacks() {
        let mut gen = XorShiftRng::from_seed([80, 2, 3, 4]);
        for _ in 0..10000 {
            let bb = generate_random_board(&mut gen);
            let b88 = BitBoard88::from(&bb);
            if bb.white_attacks() != b88.white_attacks() {
                panic!("\r\nbit-board: {:?}\r\nx88 board: {:?}\r\nfen: {}\r\n",
                       bb.white_attacks(),
                       b88.white_attacks(),
                       format!("{}", bb))
            }
        }
    }

    #[test]
    fn test_generate_random_board() {
        let mut gen = XorShiftRng::from_seed([1, 2, 3, 4]);
        assert_eq!(format!("{}", generate_random_board(&mut gen)),
                   "Nrqp4/3P4/8/8/3P4/3p4/8/8");
        assert_eq!(format!("{}", generate_random_board(&mut gen)),
                   "6p1/pb2P3/2p4N/2p2p2/3P1Pqp/r2n4/K7/5n2");
    }

    #[test]
    fn test_generate_random_board_count() {
        let mut gen = XorShiftRng::from_seed([1, 2, 3, 4]);
        assert_eq!((0..10000)
                       .into_iter()
                       .map(|_| generate_random_board(&mut gen).occupation().count())
                       .sum::<u32>(),
                   159635);
    }

    #[test]
    fn test_generate_random_board_unique_masks() {
        let mut gen = XorShiftRng::from_seed([1, 2, 3, 4]);
        for _ in 0..10000 {
            let board = generate_random_board(&mut gen);
            let fen = format!("{}", board);
            assert_eq!(BitBoard::parse(fen.as_str()), board);
        }
    }

    fn generate_random_board(rng: &mut XorShiftRng) -> BitBoard {
        let mut result = BitBoard::new();
        for one in Mask::new(rng.next_u64()).single_bits() {
            match rng.next_u32() % 38 {
                0...3 => result.set_piece(one, WHITE_PAWN),
                4 => result.set_piece(one, WHITE_KNIGHT),
                5 => result.set_piece(one, WHITE_BISHOP),
                6 => result.set_piece(one, WHITE_ROOK),
                7 => result.set_piece(one, WHITE_QUEEN),
                8 => result.set_piece(one, WHITE_KING),
                9...13 => result.set_piece(one, BLACK_PAWN),
                14 => result.set_piece(one, BLACK_KNIGHT),
                15 => result.set_piece(one, BLACK_BISHOP),
                16 => result.set_piece(one, BLACK_ROOK),
                17 => result.set_piece(one, BLACK_QUEEN),
                18 => result.set_piece(one, BLACK_KING),
                _ => {}
            }
        }
        result
    }

    use test::Bencher;

    #[bench]
    fn convert_bit_board_to_88_througn_fen(b: &mut Bencher) {
        let mut gen = XorShiftRng::from_seed([1, 2, 3, 4]);
        b.iter(|| {
            let board = generate_random_board(&mut gen);
            let fen = format!("{}", board);
            BitBoard88::parse(fen.as_str())
        });
    }
    #[bench]
    fn convert_bit_board_to_88_directly(b: &mut Bencher) {
        let mut gen = XorShiftRng::from_seed([1, 2, 3, 4]);
        b.iter(|| {
            let board = generate_random_board(&mut gen);
            BitBoard88::from(&board)
        });
    }
}
