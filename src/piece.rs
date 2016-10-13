use std::fmt::{Result, Display, Formatter};
use kind::*;
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
        if self.0 >= KINDS_COUNT {
            Color::Black
        } else {
            Color::White
        }
    }
    pub fn kind(self) -> Kind {
        if self == VOID {
            UNKNOWN
        } else {
            Kind::new(self.bits() % KINDS_COUNT)
        }
    }
    pub fn char(self) -> char {
        debug_assert!(self != VOID, "attempt to VOID.char()");
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
pub const PIECES_COUNT: usize = 12;
pub const ALL_PIECES: Piece = Piece(0);

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

impl Iterator for Piece {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < PIECES_COUNT as u8 {
            let result = *self;
            self.0 += 1;
            Some(result)
        } else {
            None
        }
    }
}

static SYMBOLS: &'static [u8; 12] = b"PNBRQKpnbrqk";

#[cfg(test)]
mod test {
    use super::*;
    use kind::*;
    use itertools::*;

    #[test]
    fn color() {
        assert_eq!(ALL_PIECES.map(|p| p.color().char())
                       .collect::<String>(),
                   "wwwwwwbbbbbb");
    }
    #[test]
    fn kind() {
        assert_eq!(VOID.kind(), UNKNOWN);
        assert_equal(ALL_KINDS.chain(ALL_KINDS), ALL_PIECES.map(Piece::kind));
    }
    #[test]
    fn display() {
        assert_eq!(ALL_PIECES.map(|pt| format!("{}", pt))
                       .collect::<String>(),
                   "PNBRQKpnbrqk");
    }
    #[test]
    fn debug() {
        assert_eq!([WHITE_PAWN, BLACK_PAWN, VOID]
                       .into_iter()
                       .map(|pt| format!("{:?}", pt))
                       .collect_vec(),
                   ["Piece(0)", "Piece(6)", "Piece(16)"]);
    }
    // noinspection SpellCheckingInspection
    #[test]
    fn parse() {
        assert_equal("PNBRQKpnbrqk"
                         .chars()
                         .into_iter()
                         .map(Piece::parse),
                     ALL_PIECES);
    }
}
