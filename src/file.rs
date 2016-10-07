use std::fmt::{Result, Display, Formatter};

#[derive(Default, Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct File(u8);

impl File {
    pub fn from_bits(bits: u8) -> Self {
        debug_assert!(bits < 8);
        File(bits)
    }
    pub fn parse(input: char) -> Self {
        debug_assert!((input as u32) < 128, "it is not even an ASCII character!");
        parse_file(&[input as u8]).unwrap().1
    }
    pub fn char(self) -> char {
        (FILES[0] + self.0) as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

static FILES: &'static [u8; 8] = b"abcdefgh";

named!(pub parse_file(&[u8]) -> File,
    map!(is_a!(FILES), |c: &[u8]| File(c[0] - FILES[0])));

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}

pub mod files {
    use super::File;

    pub const A: File = File(0);
    pub const B: File = File(1);
    pub const C: File = File(2);
    pub const D: File = File(3);
    pub const E: File = File(4);
    pub const F: File = File(5);
    pub const G: File = File(6);
    pub const H: File = File(7);

    #[derive(Copy, Clone, Debug)]
    pub struct All;

    impl IntoIterator for All {
        type Item = File;
        type IntoIter = File;

        fn into_iter(self) -> Self::IntoIter {
            File(0)
        }
    }
    impl Iterator for File {
        type Item = File;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == 8 {
                None
            } else {
                let result = *self;
                self.0 += 1;
                Some(result)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn file_char() {
        use super::files::*;

        assert_eq!(All.into_iter().
            map(|f| f.char()).collect::<Vec<_>>(),
            ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    }
    #[test]
    fn file_display() {
        use super::files::*;

        assert_eq!(All.into_iter().
            map(|f| format!("{}", f)).collect::<Vec<_>>(),
            ["a", "b", "c", "d", "e", "f", "g", "h"]);
    }
    #[test]
    fn file_debug() {
        use super::files::*;

        assert_eq!([A, H].into_iter().
            map(|f| format!("{:?}", f)).collect::<Vec<_>>(),
            ["File(0)", "File(7)"]);
    }

    #[test]
    fn file_parse() {
        use super::files::*;

        assert_eq!(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].into_iter().
            map(|f| File::parse(*f)).collect::<Vec<_>>(),
            [A, B, C, D, E, F, G, H]);
    }

}