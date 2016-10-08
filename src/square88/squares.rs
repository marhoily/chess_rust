use super::Square88;

pub const FIRST: Square88 = Square88(0);
pub const INVALID: Square88 = Square88(0xFF);

pub const A8: Square88 = Square88(0x0);
pub const B8: Square88 = Square88(0x1);
pub const C8: Square88 = Square88(0x2);
pub const D8: Square88 = Square88(0x3);
pub const E8: Square88 = Square88(0x4);
pub const F8: Square88 = Square88(0x5);
pub const G8: Square88 = Square88(0x6);
pub const H8: Square88 = Square88(0x7);
pub const A7: Square88 = Square88(0x10);
pub const B7: Square88 = Square88(0x11);
pub const C7: Square88 = Square88(0x12);
pub const D7: Square88 = Square88(0x13);
pub const E7: Square88 = Square88(0x14);
pub const F7: Square88 = Square88(0x15);
pub const G7: Square88 = Square88(0x16);
pub const H7: Square88 = Square88(0x17);
pub const A6: Square88 = Square88(0x20);
pub const B6: Square88 = Square88(0x21);
pub const C6: Square88 = Square88(0x22);
pub const D6: Square88 = Square88(0x23);
pub const E6: Square88 = Square88(0x24);
pub const F6: Square88 = Square88(0x25);
pub const G6: Square88 = Square88(0x26);
pub const H6: Square88 = Square88(0x27);
pub const A5: Square88 = Square88(0x30);
pub const B5: Square88 = Square88(0x31);
pub const C5: Square88 = Square88(0x32);
pub const D5: Square88 = Square88(0x33);
pub const E5: Square88 = Square88(0x34);
pub const F5: Square88 = Square88(0x35);
pub const G5: Square88 = Square88(0x36);
pub const H5: Square88 = Square88(0x37);
pub const A4: Square88 = Square88(0x40);
pub const B4: Square88 = Square88(0x41);
pub const C4: Square88 = Square88(0x42);
pub const D4: Square88 = Square88(0x43);
pub const E4: Square88 = Square88(0x44);
pub const F4: Square88 = Square88(0x45);
pub const G4: Square88 = Square88(0x46);
pub const H4: Square88 = Square88(0x47);
pub const A3: Square88 = Square88(0x50);
pub const B3: Square88 = Square88(0x51);
pub const C3: Square88 = Square88(0x52);
pub const D3: Square88 = Square88(0x53);
pub const E3: Square88 = Square88(0x54);
pub const F3: Square88 = Square88(0x55);
pub const G3: Square88 = Square88(0x56);
pub const H3: Square88 = Square88(0x57);
pub const A2: Square88 = Square88(0x60);
pub const B2: Square88 = Square88(0x61);
pub const C2: Square88 = Square88(0x62);
pub const D2: Square88 = Square88(0x63);
pub const E2: Square88 = Square88(0x64);
pub const F2: Square88 = Square88(0x65);
pub const G2: Square88 = Square88(0x66);
pub const H2: Square88 = Square88(0x67);
pub const A1: Square88 = Square88(0x70);
pub const B1: Square88 = Square88(0x71);
pub const C1: Square88 = Square88(0x72);
pub const D1: Square88 = Square88(0x73);
pub const E1: Square88 = Square88(0x74);
pub const F1: Square88 = Square88(0x75);
pub const G1: Square88 = Square88(0x76);
pub const H1: Square88 = Square88(0x77);

#[derive(Copy, Clone, Debug)]
pub struct All;
impl IntoIterator for All {
    type Item = Square88;
    type IntoIter = Square88;

    fn into_iter(self) -> Self::IntoIter {
        Square88(0)
    }
}
impl Iterator for Square88 {
    type Item = Square88;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 120 {
            None
        } else {
            if self.0 & 0x88 != 0 {self.0 += 8}
            let copy = *self;
            self.0 += 1;
            Some(copy)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_ranks() {
        use rank::ranks::*;

        assert_eq!(
            All.into_iter().collect::<Vec<_>>(),
            [_8, _7, _6, _5, _4, _3, _2, _1]);
    }
    #[test]
    fn all_squares() {
        assert_eq!(All.into_iter().collect::<Vec<_>>(), vec!(
            A8,B8,C8,D8,E8,F8,G8,H8,
            A7,B7,C7,D7,E7,F7,G7,H7,
            A6,B6,C6,D6,E6,F6,G6,H6,
            A5,B5,C5,D5,E5,F5,G5,H5,
            A4,B4,C4,D4,E4,F4,G4,H4,
            A3,B3,C3,D3,E3,F3,G3,H3,
            A2,B2,C2,D2,E2,F2,G2,H2,
            A1,B1,C1,D1,E1,F1,G1,H1 ));
    }

    #[test]
    fn print_const_squares() {
        for s in All {
            println!("pub const {} : Square88 = Square88({});",
                     s.to_string().to_uppercase(), s.bits());
        }
    }
    #[test]
    fn invalid() {
        assert_eq!(INVALID.is_valid(), false);
    }
    #[test]
    fn is_valid() {
        for s in All {
            assert_eq!(s.is_valid(), true);
        }
    }
}