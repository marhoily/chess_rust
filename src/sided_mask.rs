use mask::*;

#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct WhiteMask(Mask);
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct BlackMask(Mask);

pub trait SidedMask {
    fn advance(self) -> Self;
    fn attack(self) -> Self;
}
impl SidedMask for WhiteMask {
    fn advance(self) -> Self {
        WhiteMask(self.0.shift_north())
    }
    fn attack(self) -> Self {
        WhiteMask(self.0.shift_north_east() | self.0.shift_north_west())
    }
}
