use piece::*;
use mask::*;
use sided_mask::*;
use std::ops::*;

pub trait Side {
    type Mask : SidedMask;
    const PAWN : Piece;
    const RANGE : Range<usize>;
}
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct White;
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct Black;

impl Side for White {
    type Mask = WhiteMask;
    const PAWN : Piece = WHITE_PAWN;
    const RANGE : Range<usize> = 0..6;
}
impl Side for Black {
    type Mask = BlackMask;
    const PAWN : Piece = BLACK_PAWN;
    const RANGE : Range<usize> = 6..12;
}
