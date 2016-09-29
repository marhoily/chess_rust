#![allow(dead_code)]

use moves::*;
use sqares::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PieceTypeBits(u64);

#[derive(Debug, PartialEq)]
pub struct BitBoard([PieceTypeBits; PIECES_COUNT]);

impl PieceTypeBits {
    fn test(self, square: SquareExp) -> bool {
        self.0 & square.bits() != 0
    }
    fn set(&mut self, square: SquareExp) {
        self.0 |= square.bits()
    }
    fn count(self) -> u32 {
        self.0.count_ones()
    }
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard([PieceTypeBits(0); PIECES_COUNT])
    }
    pub fn for_piece(&self, piece: Piece) -> PieceTypeBits {
        self.0[piece.bits() as usize]
    }
    pub fn check_square(&self, square: SquareExp) -> Piece {
        for piece in AllPieces {
            if self.for_piece(piece).test(square) {
                return piece;
            }
        }
        pieces::EMPTY
    }
    pub fn set_piece(&mut self, square: SquareExp, piece: Piece) {
        self.0[piece.bits() as usize].0 |= square.bits();
    }
    pub fn get_piece(&self, square: SquareExp) -> Piece {
        for probe in AllPieces {
            if self.for_piece(probe).test(square) {
                return probe;
            }
        }
        pieces::EMPTY
    }
    pub fn squares<'a>(&'a self) -> SquareIter<'a> {
        SquareIter {
            board: &self,
            square_iter: SquareExpIter::new(),
        }
    }
}

pub struct SquareIter<'a> {
    board: &'a BitBoard,
    square_iter: SquareExpIter,
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        self.square_iter.next().map(|square| self.board.get_piece(square))
    }
}

bitflags! {
    pub flags Castling: u8 {
        //const None = 0,
        const Q = WQ.bits | BQ.bits,
        const K = WK.bits | BK.bits,
        const W = WQ.bits | WK.bits,
        const B = BQ.bits | BK.bits,
        const WQ = 1 << 0,
        const WK = 1 << 2,
        const BQ = 1 << 3,
        const BK = 1 << 4,
        const ALL = Q.bits | K.bits,
    }
}

enum MoveAnnotations {
    None,
    Promotion,
    Capture,
    EnPassant,
    DoublePush,
}

enum Warnings {
    None,
    MissingPromotionHint,
    SparePromotion,
}

enum Errors {
    None,

    MoveToCheck,
    FromEmptyCell,
    ToOccupiedCell,
    WrongSideToMove,

    CastleFromCheck,
    CastleThroughCheck,
    HasNoCastling,

    HasNoEnPassant,

    DoesNotMoveThisWay,
    DoesNotCaptureThisWay,
    OnlyCapturesThisWay,
    JumpsOverPieces,
}

struct Position {
    board: BitBoard,
    active: Color,
    available: Castling,
    en_passant: Option<File>,
}

#[cfg(test)]
mod test {
    use sqares::*;
    use super::*;
    use std::iter::*;
    use moves::*;
    use moves::pieces::*;

    #[test]
    fn check_square() {
        let mut b = BitBoard::new();
        b.set_piece(SquareExp::new(0b0001), BLACK_ROOK);
        b.set_piece(SquareExp::new(0b0100), BLACK_ROOK);
        assert_eq!(b.check_square(SquareExp::new(0b0001)), BLACK_ROOK);
        assert_eq!(b.check_square(SquareExp::new(0b0001)), BLACK_ROOK);
    }

    #[test]
    fn bit_board_squares() {
        let mut b = BitBoard::new();
        b.set_piece(SquareExp::new(0b0001), BLACK_ROOK);

        let all = b.squares()
            .collect::<Vec<Piece>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], BLACK_ROOK);
        assert_eq!(all[63], EMPTY);
    }
}
