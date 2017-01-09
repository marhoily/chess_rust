use piece::*;
use mask::*;
use sided_mask::*;
use std::ops::*;
use rank::*;

pub trait Side {
    type Mask : SidedMask;
    type Opposite : Side;
    const PAWN : Piece;
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
    const RANGE : Range<usize> = 0..6;
    const EN_PASSANT_RANK : Rank = _6;
}
impl Side for Black {
    type Mask = BlackMask;
    type Opposite = White;
    const PAWN : Piece = BLACK_PAWN;
    const RANGE : Range<usize> = 6..12;
    const EN_PASSANT_RANK : Rank = _3;
}
