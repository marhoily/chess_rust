use piece::*;
use mask::*;
use sided_mask::*;
use std::ops::*;
use rank::*;

pub trait Side {
    type Mask : SidedMask;
    type Opposite : Side;

    const PAWN : Piece;
    const KNIGHT : Piece;
    const BISHOP : Piece;
    const ROOK : Piece;
    const QUEEN : Piece;
    const KING : Piece;

    const RANGE : Range<usize>;
    const EN_PASSANT_RANK : Rank;
}

#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct White;
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct Black;

impl Side for White {
    type Mask = WhiteMask;
    type Opposite = Black;
    const PAWN : Piece = WHITE_PAWN;
    const KNIGHT : Piece = WHITE_KNIGHT;
    const BISHOP : Piece = WHITE_BISHOP;
    const ROOK : Piece = WHITE_ROOK;
    const QUEEN : Piece = WHITE_QUEEN;
    const KING : Piece = WHITE_KING;
    
    const RANGE : Range<usize> = 0..6;
    const EN_PASSANT_RANK : Rank = _6;
}
impl Side for Black {
    type Mask = BlackMask;
    type Opposite = White;
    
    const PAWN : Piece = BLACK_PAWN;
    const KNIGHT : Piece = BLACK_KNIGHT;
    const BISHOP : Piece = BLACK_BISHOP;
    const ROOK : Piece = BLACK_ROOK;
    const QUEEN : Piece = BLACK_QUEEN;
    const KING : Piece = BLACK_KING;

    const RANGE : Range<usize> = 6..12;
    const EN_PASSANT_RANK : Rank = _3;
}
