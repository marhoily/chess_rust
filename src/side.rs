use piece::*;

pub trait Side {
    const PAWN : Piece;
}
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct White;
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct Black;

impl Side for White {
    const PAWN : Piece = WHITE_PAWN;
}
impl Side for Black {
    const PAWN : Piece = BLACK_PAWN;
}
