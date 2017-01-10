use super::*;
pub trait SidedMove {}

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq)]
pub struct WhiteMove(Move);

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq)]
pub struct BlackMove(Move);

impl SidedMove for WhiteMove {}

impl SidedMove for BlackMove {}


