use std::fmt::{Result, Display, Formatter};
use kind::{Kind, kinds};
use color::Color;

#[derive(Eq, Copy, Clone, Debug, PartialEq, Hash)]
pub struct Piece(u8);

impl Piece {
    pub fn new(bits: u8) -> Self {
        Piece(bits)
    }
    pub fn parse(input: char) -> Self {
        parse_piece(&[input as u8]).unwrap().1
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn color(self) -> Color {
        if self.0 >= kinds::COUNT {
            Color::Black
        } else {
            Color::White
        }
    }
    pub fn kind(self) -> Kind {
        if self == pieces::VOID {
            kinds::UNKNOWN
        } else {
            Kind::new(self.bits() % kinds::COUNT)
        }
    }
    pub fn char(self) -> char {
        debug_assert!(self != pieces::VOID, "attempt to pieces::VOID.char()");
        SYMBOLS[self.0 as usize] as char
    }
}

named!(parse_piece(&[u8]) -> Piece,
    map!(is_a!(SYMBOLS), |c: &[u8]| {
            let idx = SYMBOLS.iter().position(|x| *x == c[0]);
            Piece(idx.unwrap() as u8)
        }));

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.char())
    }
}
use std::ops::Range;
pub const PIECES_RANGE :Range<u8> = 0..12;
pub const WHITE_RANGE :Range<Piece> = pieces::WHITE_PAWN..pieces::BLACK_PAWN;

pub mod pieces {
    use super::Piece;

    pub const COUNT: usize = 12;

    pub const WHITE_PAWN: Piece = Piece(0);
    pub const WHITE_KNIGHT: Piece = Piece(1);
    pub const WHITE_BISHOP: Piece = Piece(2);
    pub const WHITE_ROOK: Piece = Piece(3);
    pub const WHITE_QUEEN: Piece = Piece(4);
    pub const WHITE_KING: Piece = Piece(5);
    pub const BLACK_PAWN: Piece = Piece(6);
    pub const BLACK_KNIGHT: Piece = Piece(7);
    pub const BLACK_BISHOP: Piece = Piece(8);
    pub const BLACK_ROOK: Piece = Piece(9);
    pub const BLACK_QUEEN: Piece = Piece(10);
    pub const BLACK_KING: Piece = Piece(11);
    pub const VOID: Piece = Piece(16);

    #[derive(Copy, Clone, Debug)]
    pub struct Pieces;

    impl IntoIterator for Pieces {
        type Item = Piece;
        type IntoIter = Piece;

        fn into_iter(self) -> Self::IntoIter {
            Piece(0)
        }
    }

    impl Iterator for Piece {
        type Item = Piece;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 < COUNT as u8 {
                let result = *self;
                self.0 += 1;
                Some(result)
            } else {
                None
            }
        }
    }
}

static SYMBOLS: &'static [u8; 12] = b"PNBRQKpnbrqk";

#[cfg(test)]
mod test {
    use super::*;
    use super::pieces::*;
    use std::iter::*;
    use kind::kinds;

    #[test]
    fn color() {
        use color::Color::*;

        assert_eq!(Pieces.into_iter().map(Piece::color).collect::<Vec<_>>(),
                   [White, White, White, White, White, White, Black, Black, Black, Black, Black,
                    Black]);
    }
    #[test]
    fn kind() {
        assert_eq!(VOID.kind(), kinds::UNKNOWN);

        assert_eq!(kinds::All.into_iter().chain(kinds::All.into_iter()).collect::<Vec<_>>(),
                   Pieces.into_iter().map(Piece::kind).collect::<Vec<_>>());
    }
    #[test]
    fn display() {
        assert_eq!(Pieces.into_iter()
                       .map(|pt| format!("{}", pt))
                       .collect::<String>(),
                   "PNBRQKpnbrqk");
    }
    #[test]
    fn debug() {
        use super::pieces::*;

        assert_eq!([WHITE_PAWN, BLACK_PAWN, VOID]
                       .into_iter()
                       .map(|pt| format!("{:?}", pt))
                       .collect::<Vec<_>>(),
                   ["Piece(0)", "Piece(6)", "Piece(16)"]);
    }
    // noinspection SpellCheckingInspection
    #[test]
    fn parse() {
        use super::pieces::*;

        assert_eq!("PNBRQKpnbrqk"
                       .chars()
                       .into_iter()
                       .map(Piece::parse)
                       .collect::<Vec<_>>(),
                   [WHITE_PAWN,
                    WHITE_KNIGHT,
                    WHITE_BISHOP,
                    WHITE_ROOK,
                    WHITE_QUEEN,
                    WHITE_KING,
                    BLACK_PAWN,
                    BLACK_KNIGHT,
                    BLACK_BISHOP,
                    BLACK_ROOK,
                    BLACK_QUEEN,
                    BLACK_KING]);
    }
}
