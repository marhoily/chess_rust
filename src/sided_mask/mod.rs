use mask::*;
use masks::*;

#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct WhiteMask(pub Mask);
#[derive(Eq, Copy, Clone, Debug, Default, PartialEq)]
pub struct BlackMask(pub Mask);

pub mod ops_none_on_white;
pub mod ops_white_on_white;
pub mod ops_black_on_black;

pub trait SidedMask {
    fn wrap(m: Mask) -> Self;
    fn advance(self) -> Self;
    fn attack(self) -> Self;
    fn pawn_attacks_and_pushes(self, stoppers: Mask) -> Self;
    fn pawn_double_pushes(self, stoppers: Mask) -> Self;
}

impl SidedMask for WhiteMask {
    fn advance(self) -> Self {
        WhiteMask(self.0.shift_north())
    }
    fn attack(self) -> Self {
        WhiteMask(self.0.shift_north_east() | self.0.shift_north_west())
    }
    fn wrap(m: Mask) -> Self {
        WhiteMask(m)
    }
    fn pawn_attacks_and_pushes(self, stoppers: Mask) -> Self {
        self.attack() | self.advance() | self.pawn_double_pushes(stoppers)
    }
    fn pawn_double_pushes(self, stoppers: Mask) -> Self {
        let first_push = (self.0 & _2).shift_north();
        WhiteMask(first_push | (first_push & !stoppers).shift_north())
    }
}
impl SidedMask for BlackMask {
    fn advance(self) -> Self {
        BlackMask(self.0.shift_north())
    }
    fn attack(self) -> Self {
        BlackMask(self.0.shift_north_east() | self.0.shift_north_west())
    }
    fn wrap(m: Mask) -> Self {
        BlackMask(m)
    }
    fn pawn_attacks_and_pushes(self, stoppers: Mask) -> Self {
        unimplemented!()
    }
    fn pawn_double_pushes(self, stoppers: Mask) -> Self {
        unimplemented!()
    }
}
