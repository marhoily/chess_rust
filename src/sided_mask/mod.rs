use mask::*;
use masks::*;

#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct WhiteMask(pub Mask);
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct BlackMask(pub Mask);

pub mod ops_black_on_white;
pub mod ops_white_on_black;
pub mod ops_none_on_white;
pub mod ops_white_on_white;
pub mod ops_black_on_black;

pub trait SidedMask : Into<Mask>
    where Self: Sized
{
    fn wrap(m: Mask) -> Self;
    fn mask(&self) -> Mask;
    fn advance(&self) -> Self;
    fn attack(&self) -> Self;

    fn filter<M: Into<Mask>>(&self, f: M) -> Self {
        Self::wrap(self.mask() & f.into())
    }
    fn and<M: Into<Mask>>(&self, f: M) -> Self {
        Self::wrap(self.mask() | f.into())
    }
}
impl Into<Mask> for WhiteMask {
    fn into(self) -> Mask {
        self.0
    }
}
impl Into<Mask> for BlackMask {
    fn into(self) -> Mask {
        self.0
    }
}
impl SidedMask for WhiteMask {
    fn advance(&self) -> Self {
        WhiteMask(self.0.shift_north())
    }
    fn attack(&self) -> Self {
        WhiteMask(self.0.shift_north_east() | self.0.shift_north_west())
    }
    fn wrap(m: Mask) -> Self {
        WhiteMask(m)
    }

    fn mask(&self) -> Mask {
        self.0
    }
}
impl SidedMask for BlackMask {
    fn advance(&self) -> Self {
        BlackMask(self.0.shift_south())
    }
    fn attack(&self) -> Self {
        BlackMask(self.0.shift_south_east() | self.0.shift_south_west())
    }
    fn wrap(m: Mask) -> Self {
        BlackMask(m)
    }
    fn mask(&self) -> Mask {
        self.0
    }
}
