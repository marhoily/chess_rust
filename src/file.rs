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
        FILE_SYMBOLS[self.0 as usize] as char
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}

static FILE_SYMBOLS: &'static [u8; 8] = b"abcdefgh";

named!(pub parse_file(&[u8]) -> File,
    map!(is_a!(FILE_SYMBOLS), |c: &[u8]| {
        File(c[0] - FILE_SYMBOLS[0])
    }));

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}

pub const ALL_FILES: File = File(0);

pub const A: File = File(0);
pub const B: File = File(1);
pub const C: File = File(2);
pub const D: File = File(3);
pub const E: File = File(4);
pub const F: File = File(5);
pub const G: File = File(6);
pub const H: File = File(7);

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

#[cfg(test)]
mod test {
    use super::*;
    use itertools::*;

    #[test]
    fn all_files() {
        assert_eq!(ALL_FILES.collect_vec(), [A, B, C, D, E, F, G, H]);
    }
    #[test]
    fn file_char() {
        assert_eq!(ALL_FILES.map(|f| f.char()).collect::<String>(), "abcdefgh");
    }
    #[test]
    fn file_display() {
        assert_eq!(ALL_FILES.map(|f| format!("{}", f)).join(""), "abcdefgh");
    }
    #[test]
    fn file_debug() {
        assert_eq!(format!("{:?}", A), "File(0)");
        assert_eq!(format!("{:?}", H), "File(7)");
    }

    #[test]
    fn file_parse() {
        assert_eq!(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].into_iter().
            map(|f| File::parse(*f)).collect_vec(),
            [A, B, C, D, E, F, G, H]);
    }
}
