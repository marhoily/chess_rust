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

#[cfg(test)]
mod tests {
    use mask::*;
    use super::*;
    use super::super::*;

    pub fn none() -> Mask {
        Mask::new(0)
    }
    pub fn white() -> WhiteMask {
        WhiteMask::wrap(none())
    }
    pub fn is_mask(m : Mask) {

    }

    #[test]
    fn white_bit_or_none() {
        is_mask(white() | none());
    }
    #[test]
    fn none_bit_or_white() {
        is_mask(none() | white());
    }
}