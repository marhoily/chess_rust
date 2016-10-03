use super::*;
use std::ops::*;

impl BitOr<Mask> for Mask {
    type Output = Mask;
    fn bitor(self, rhs: Mask) -> Self::Output {
        Mask(self.0 | rhs.0)
    }
}
impl BitOrAssign<Mask> for Mask {
    fn bitor_assign(&mut self, rhs: Mask) {
        self.0 |= rhs.0
    }
}
impl BitAnd<Mask> for Mask {
    type Output = Mask;
    fn bitand(self, rhs: Mask) -> Self::Output {
        Mask(self.0 & rhs.0)
    }
}
impl BitAndAssign<Mask> for Mask {
    fn bitand_assign(&mut self, rhs: Mask) {
        self.0 &= rhs.0
    }
}
impl BitXor<Mask> for Mask {
    type Output = Mask;
    fn bitxor(self, rhs: Mask) -> Self::Output {
        Mask(self.0 & rhs.0)
    }
}
impl BitXorAssign<Mask> for Mask {
    fn bitxor_assign(&mut self, rhs: Mask) {
        self.0 &= rhs.0
    }
}
impl Shl<u8> for Mask {
    type Output = Mask;
    fn shl(self, rhs: u8) -> Self::Output {
        Mask(self.0 << rhs)
    }
}
impl ShlAssign<u8> for Mask {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}
impl Shr<u8> for Mask {
    type Output = Mask;
    fn shr(self, rhs: u8) -> Self::Output {
        Mask(self.0 >> rhs)
    }
}
impl ShrAssign<u8> for Mask {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}
impl Not for Mask {
    type Output = Mask;
    fn not(self) -> Self::Output {
        Mask(!self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::super::masks::*;

    #[test]
    fn shr() {
        assert_eq!(A8 >> 1, EMPTY);
    }
}
