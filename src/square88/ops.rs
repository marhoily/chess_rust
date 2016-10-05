use super::*;
use std::ops::*;

impl BitOr<Square88> for Square88 {
    type Output = Square88;
    fn bitor(self, rhs: Square88) -> Self::Output {
        Square88(self.0 | rhs.0)
    }
}
impl BitOrAssign<Square88> for Square88 {
    fn bitor_assign(&mut self, rhs: Square88) {
        self.0 |= rhs.0
    }
}
impl BitAnd<Square88> for Square88 {
    type Output = Square88;
    fn bitand(self, rhs: Square88) -> Self::Output {
        Square88(self.0 & rhs.0)
    }
}
impl BitAndAssign<Square88> for Square88 {
    fn bitand_assign(&mut self, rhs: Square88) {
        self.0 &= rhs.0
    }
}
impl BitXor<Square88> for Square88 {
    type Output = Square88;
    fn bitxor(self, rhs: Square88) -> Self::Output {
        Square88(self.0 & rhs.0)
    }
}
impl BitXorAssign<Square88> for Square88 {
    fn bitxor_assign(&mut self, rhs: Square88) {
        self.0 &= rhs.0
    }
}
impl Shl<u8> for Square88 {
    type Output = Square88;
    fn shl(self, rhs: u8) -> Self::Output {
        Square88(self.0 << rhs)
    }
}
impl ShlAssign<u8> for Square88 {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}
impl Shr<u8> for Square88 {
    type Output = Square88;
    fn shr(self, rhs: u8) -> Self::Output {
        Square88(self.0 >> rhs)
    }
}
impl ShrAssign<u8> for Square88 {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}
impl Not for Square88 {
    type Output = Square88;
    fn not(self) -> Self::Output {
        Square88(!self.0)
    }
}

impl Add<i8> for Square88 {
    type Output = Square88;

    fn add(self, rhs: i8) -> Self::Output {
        Square88((self.0 as i8 + rhs) as u8)
    }
}

