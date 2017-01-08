use super::*;
use std::ops::*;

impl BitOr<BlackMask> for BlackMask {
    type Output = BlackMask;
    fn bitor(self, rhs: BlackMask) -> Self::Output {
        BlackMask(self.0 | rhs.0)
    }
}
impl BitOrAssign<BlackMask> for BlackMask {
    fn bitor_assign(&mut self, rhs: BlackMask) {
        self.0 |= rhs.0
    }
}
impl BitAnd<BlackMask> for BlackMask {
    type Output = BlackMask;
    fn bitand(self, rhs: BlackMask) -> Self::Output {
        BlackMask(self.0 & rhs.0)
    }
}
impl BitAndAssign<BlackMask> for BlackMask {
    fn bitand_assign(&mut self, rhs: BlackMask) {
        self.0 &= rhs.0
    }
}
impl BitXor<BlackMask> for BlackMask {
    type Output = BlackMask;
    fn bitxor(self, rhs: BlackMask) -> Self::Output {
        BlackMask(self.0 & rhs.0)
    }
}
impl BitXorAssign<BlackMask> for BlackMask {
    fn bitxor_assign(&mut self, rhs: BlackMask) {
        self.0 &= rhs.0
    }
}
impl Shl<u8> for BlackMask {
    type Output = BlackMask;
    fn shl(self, rhs: u8) -> Self::Output {
        BlackMask(self.0 << rhs)
    }
}
impl ShlAssign<u8> for BlackMask {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}
impl Shr<u8> for BlackMask {
    type Output = BlackMask;
    fn shr(self, rhs: u8) -> Self::Output {
        BlackMask(self.0 >> rhs)
    }
}
impl ShrAssign<u8> for BlackMask {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}
impl Not for BlackMask {
    type Output = BlackMask;
    fn not(self) -> Self::Output {
        BlackMask(!self.0)
    }
}


