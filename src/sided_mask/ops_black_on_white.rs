use super::*;
use std::ops::*;

impl BitAnd<BlackMask> for WhiteMask {
    type Output = Mask;
    fn bitand(self, rhs: BlackMask) -> Self::Output {
        self.0 & rhs.0
    }
}
impl BitAnd<WhiteMask> for BlackMask {
    type Output = Mask;
    fn bitand(self, rhs: WhiteMask) -> Self::Output {
        self.0 & rhs.0
    }
}
