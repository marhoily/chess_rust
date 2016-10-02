use super::Mask;

pub const EMPTY: Mask = Mask(0);

pub mod files {
    use super::super::Mask;

    pub const A: Mask = Mask(0x101010101010101);
    pub const B: Mask = Mask(0x202020202020202);
    pub const C: Mask = Mask(0x404040404040404);
    pub const D: Mask = Mask(0x808080808080808);
    pub const E: Mask = Mask(0x1010101010101010);
    pub const F: Mask = Mask(0x2020202020202020);
    pub const G: Mask = Mask(0x4040404040404040);
    pub const H: Mask = Mask(0x8080808080808080);

    pub static ALL: &'static [Mask] = &[A, B, C, D, E, F, G, H];
}
pub mod ranks {
    use super::super::Mask;

    pub const _8: Mask = Mask(0xFF);
    pub const _7: Mask = Mask(0xFF00);
    pub const _6: Mask = Mask(0xFF0000);
    pub const _5: Mask = Mask(0xFF000000);
    pub const _4: Mask = Mask(0xFF00000000);
    pub const _3: Mask = Mask(0xFF0000000000);
    pub const _2: Mask = Mask(0xFF000000000000);
    pub const _1: Mask = Mask(0xFF00000000000000);

    pub static ALL: &'static [Mask] = &[_8, _7, _6, _5, _4, _3, _2, _1];
}

pub const A8: Mask = Mask(0x1);
pub const B8: Mask = Mask(0x2);
pub const C8: Mask = Mask(0x4);
pub const D8: Mask = Mask(0x8);
pub const E8: Mask = Mask(0x10);
pub const F8: Mask = Mask(0x20);
pub const G8: Mask = Mask(0x40);
pub const H8: Mask = Mask(0x80);
pub const A7: Mask = Mask(0x100);
pub const B7: Mask = Mask(0x200);
pub const C7: Mask = Mask(0x400);
pub const D7: Mask = Mask(0x800);
pub const E7: Mask = Mask(0x1000);
pub const F7: Mask = Mask(0x2000);
pub const G7: Mask = Mask(0x4000);
pub const H7: Mask = Mask(0x8000);
pub const A6: Mask = Mask(0x10000);
pub const B6: Mask = Mask(0x20000);
pub const C6: Mask = Mask(0x40000);
pub const D6: Mask = Mask(0x80000);
pub const E6: Mask = Mask(0x100000);
pub const F6: Mask = Mask(0x200000);
pub const G6: Mask = Mask(0x400000);
pub const H6: Mask = Mask(0x800000);
pub const A5: Mask = Mask(0x1000000);
pub const B5: Mask = Mask(0x2000000);
pub const C5: Mask = Mask(0x4000000);
pub const D5: Mask = Mask(0x8000000);
pub const E5: Mask = Mask(0x10000000);
pub const F5: Mask = Mask(0x20000000);
pub const G5: Mask = Mask(0x40000000);
pub const H5: Mask = Mask(0x80000000);
pub const A4: Mask = Mask(0x100000000);
pub const B4: Mask = Mask(0x200000000);
pub const C4: Mask = Mask(0x400000000);
pub const D4: Mask = Mask(0x800000000);
pub const E4: Mask = Mask(0x1000000000);
pub const F4: Mask = Mask(0x2000000000);
pub const G4: Mask = Mask(0x4000000000);
pub const H4: Mask = Mask(0x8000000000);
pub const A3: Mask = Mask(0x10000000000);
pub const B3: Mask = Mask(0x20000000000);
pub const C3: Mask = Mask(0x40000000000);
pub const D3: Mask = Mask(0x80000000000);
pub const E3: Mask = Mask(0x100000000000);
pub const F3: Mask = Mask(0x200000000000);
pub const G3: Mask = Mask(0x400000000000);
pub const H3: Mask = Mask(0x800000000000);
pub const A2: Mask = Mask(0x1000000000000);
pub const B2: Mask = Mask(0x2000000000000);
pub const C2: Mask = Mask(0x4000000000000);
pub const D2: Mask = Mask(0x8000000000000);
pub const E2: Mask = Mask(0x10000000000000);
pub const F2: Mask = Mask(0x20000000000000);
pub const G2: Mask = Mask(0x40000000000000);
pub const H2: Mask = Mask(0x80000000000000);
pub const A1: Mask = Mask(0x100000000000000);
pub const B1: Mask = Mask(0x200000000000000);
pub const C1: Mask = Mask(0x400000000000000);
pub const D1: Mask = Mask(0x800000000000000);
pub const E1: Mask = Mask(0x1000000000000000);
pub const F1: Mask = Mask(0x2000000000000000);
pub const G1: Mask = Mask(0x4000000000000000);
pub const H1: Mask = Mask(0x8000000000000000);

#[derive(Copy, Clone, Debug)]
pub struct All;
impl IntoIterator for All {
    type Item = Mask;
    type IntoIter = MaskIter;

    fn into_iter(self) -> Self::IntoIter {
        MaskIter::new()
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct MaskIter(u64);

impl MaskIter {
    pub fn new() -> Self {
        MaskIter(1)
    }
}

impl Iterator for MaskIter {
    type Item = Mask;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let result = Mask(self.0);
            self.0 <<= 1;
            Some(result)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Mask;
    use std::iter::*;

    #[test]
    fn all_squares() {
        assert_eq!(All.into_iter().collect::<Vec<_>>(),
                   vec![A8, B8, C8, D8, E8, F8, G8, H8, A7, B7, C7, D7, E7, F7, G7, H7, A6, B6,
                        C6, D6, E6, F6, G6, H6, A5, B5, C5, D5, E5, F5, G5, H5, A4, B4, C4, D4,
                        E4, F4, G4, H4, A3, B3, C3, D3, E3, F3, G3, H3, A2, B2, C2, D2, E2, F2,
                        G2, H2, A1, B1, C1, D1, E1, F1, G1, H1]);
    }

    fn split(m: Mask) -> Vec<Mask> {
        m.iter_bit_masks().collect::<Vec<_>>()
    }

    #[test]
    fn files() {
        assert_eq!(split(files::A), [A8, A7, A6, A5, A4, A3, A2, A1]);
        assert_eq!(split(files::B), [B8, B7, B6, B5, B4, B3, B2, B1]);
        assert_eq!(split(files::C), [C8, C7, C6, C5, C4, C3, C2, C1]);
        assert_eq!(split(files::D), [D8, D7, D6, D5, D4, D3, D2, D1]);
        assert_eq!(split(files::E), [E8, E7, E6, E5, E4, E3, E2, E1]);
        assert_eq!(split(files::F), [F8, F7, F6, F5, F4, F3, F2, F1]);
        assert_eq!(split(files::G), [G8, G7, G6, G5, G4, G3, G2, G1]);
        assert_eq!(split(files::H), [H8, H7, H6, H5, H4, H3, H2, H1]);
    }
    #[test]
    fn ranks() {
        assert_eq!(split(ranks::_8), [A8, B8, C8, D8, E8, F8, G8, H8]);
        assert_eq!(split(ranks::_7), [A7, B7, C7, D7, E7, F7, G7, H7]);
        assert_eq!(split(ranks::_6), [A6, B6, C6, D6, E6, F6, G6, H6]);
        assert_eq!(split(ranks::_5), [A5, B5, C5, D5, E5, F5, G5, H5]);
        assert_eq!(split(ranks::_4), [A4, B4, C4, D4, E4, F4, G4, H4]);
        assert_eq!(split(ranks::_3), [A3, B3, C3, D3, E3, F3, G3, H3]);
        assert_eq!(split(ranks::_2), [A2, B2, C2, D2, E2, F2, G2, H2]);
        assert_eq!(split(ranks::_1), [A1, B1, C1, D1, E1, F1, G1, H1]);
    }

    #[test]
    fn print_const_files() {
        use geometry::Square;
        use std::ascii::AsciiExt;

        for file in ::geometry::files::All {
            let mut mask = EMPTY;
            for rank in ::geometry::ranks::All {
                mask |= Square::from(file, rank).mask();
            }
            println!("pub const {} : Mask = Mask(0x{:X});",
                     file.char().to_ascii_uppercase(),
                     mask.bits());
        }
    }
    #[test]
    fn print_const_ranks() {
        use geometry::Square;

        for rank in ::geometry::ranks::All {
            let mut mask = EMPTY;
            for file in ::geometry::files::All {
                mask |= Square::from(file, rank).mask();
            }
            println!("pub const _{} : Mask = Mask(0x{:X});", rank, mask.bits());
        }
    }
    #[test]
    fn print_const_squares() {
        for s in ::geometry::squares::All {
            println!("pub const {} : Mask = Mask(0x{:X});",
                     s.to_string().to_uppercase(),
                     s.mask().bits());
        }
    }
}
