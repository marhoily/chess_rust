use piece::*;
use mask::*;
use sided_mask::*;

pub trait Side {
    type Mask : SidedMask;
    const PAWN : Piece;
}
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct White;
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct Black;

impl Side for White {
    type Mask = WhiteMask;
    const PAWN : Piece = WHITE_PAWN;
}
impl Side for Black {
    type Mask = BlackMask;
    const PAWN : Piece = BLACK_PAWN;
}
