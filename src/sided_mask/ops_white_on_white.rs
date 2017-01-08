use super::*;
use std::ops::*;



impl BitOr<WhiteMask> for WhiteMask {
    type Output = WhiteMask;
    fn bitor(self, rhs: WhiteMask) -> Self::Output {
        WhiteMask(self.0 | rhs.0)
    }
}
impl BitOrAssign<WhiteMask> for WhiteMask {
    fn bitor_assign(&mut self, rhs: WhiteMask) {
        self.0 |= rhs.0
    }
}
impl BitAnd<WhiteMask> for WhiteMask {
    type Output = WhiteMask;
    fn bitand(self, rhs: WhiteMask) -> Self::Output {
        WhiteMask(self.0 & rhs.0)
    }
}
impl BitAndAssign<WhiteMask> for WhiteMask {
    fn bitand_assign(&mut self, rhs: WhiteMask) {
        self.0 &= rhs.0
    }
}
impl BitXor<WhiteMask> for WhiteMask {
    type Output = WhiteMask;
    fn bitxor(self, rhs: WhiteMask) -> Self::Output {
        WhiteMask(self.0 & rhs.0)
    }
}
impl BitXorAssign<WhiteMask> for WhiteMask {
    fn bitxor_assign(&mut self, rhs: WhiteMask) {
        self.0 &= rhs.0
    }
}
impl Shl<u8> for WhiteMask {
    type Output = WhiteMask;
    fn shl(self, rhs: u8) -> Self::Output {
        WhiteMask(self.0 << rhs)
    }
}
impl ShlAssign<u8> for WhiteMask {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}
impl Shr<u8> for WhiteMask {
    type Output = WhiteMask;
    fn shr(self, rhs: u8) -> Self::Output {
        WhiteMask(self.0 >> rhs)
    }
}
impl ShrAssign<u8> for WhiteMask {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}
impl Not for WhiteMask {
    type Output = WhiteMask;
    fn not(self) -> Self::Output {
        WhiteMask(!self.0)
    }
}


