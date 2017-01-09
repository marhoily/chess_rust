use super::*;
use std::ops::*;

impl BitOr<Mask> for WhiteMask {
    type Output = Mask;
    fn bitor(self, rhs: Mask) -> Self::Output {
        self.0 | rhs
    }
}
impl BitOr<WhiteMask> for Mask {
    type Output = Mask;
    fn bitor(self, rhs: WhiteMask) -> Self::Output {
        self | rhs.0
    }
}
impl BitAnd<Mask> for WhiteMask {
    type Output = WhiteMask;
    fn bitand(self, rhs: Mask) -> Self::Output {
        WhiteMask(self.0 & rhs)
    }
}
impl BitAnd<WhiteMask> for Mask {
    type Output = WhiteMask;
    fn bitand(self, rhs: WhiteMask) -> Self::Output {
        WhiteMask(self & rhs.0)
    }
}
impl BitAnd<Mask> for BlackMask {
    type Output = BlackMask;
    fn bitand(self, rhs: Mask) -> Self::Output {
        BlackMask(self.0 & rhs)
    }
}
impl BitAnd<BlackMask> for Mask {
    type Output = BlackMask;
    fn bitand(self, rhs: BlackMask) -> Self::Output {
        BlackMask(self & rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use mask::masks::*;
    use super::*;

    pub fn none() -> Mask {
        E
    }
    pub fn white() -> WhiteMask {
        WhiteMask::wrap(_4)
    }

    #[test]
    fn white_bit_or_none() {
        assert_eq!(white() | none(), E | _4);
    }
    #[test]
    fn none_bit_or_white() {
        assert_eq!(none() | white(), E | _4);
    }
}