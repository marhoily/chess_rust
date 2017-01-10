use super::*;
use kind::*;
use square::*;
use castle::Castle;

pub trait SidedMove {
    fn from(&self) -> Square;
    fn to(&self) -> Square;
    fn promote(&self) -> Kind;
    fn castle(&self) -> Castle;
}

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq)]
pub struct WhiteMove(pub Move);

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq)]
pub struct BlackMove(pub Move);

impl SidedMove for WhiteMove {
    fn from(&self) -> Square{
        self.0.from
    }
    fn to(&self) -> Square{
        self.0.to
    }
    fn promote(&self) -> Kind{
        self.0.promote

    }
    fn castle(&self) -> Castle{
        self.0.castle

    }

}

impl SidedMove for BlackMove {
    fn from(&self) -> Square{
        self.0.from
    }
    fn to(&self) -> Square{
        self.0.to
    }
    fn promote(&self) -> Kind{
        self.0.promote

    }
    fn castle(&self) -> Castle{
        self.0.castle

    }
}


