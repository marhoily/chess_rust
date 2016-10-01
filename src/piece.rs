use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use geometry::*;
use kind::{Kind, kinds};

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
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
        Piece(SYMBOLS.iter().position(|x| {
            *x == c[0]}).unwrap() as u8)}));

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self == pieces::VOID {
            write!(f, "Void")
        } else {
            write!(f, "{:?}-{}", self.color(), self.kind())
        }
    }
}

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
    pub struct All;

    impl IntoIterator for All {
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
    use super::pieces::*;
    use std::iter::*;
    use geometry::*;
    use kind::kinds;

    #[test]
    fn piece_color() {
        assert_eq!(WHITE_PAWN.color(), Color::White);
        assert_eq!(WHITE_KNIGHT.color(), Color::White);
        assert_eq!(WHITE_BISHOP.color(), Color::White);
        assert_eq!(WHITE_ROOK.color(), Color::White);
        assert_eq!(WHITE_QUEEN.color(), Color::White);
        assert_eq!(WHITE_KING.color(), Color::White);
        assert_eq!(BLACK_PAWN.color(), Color::Black);
        assert_eq!(BLACK_KNIGHT.color(), Color::Black);
        assert_eq!(BLACK_BISHOP.color(), Color::Black);
        assert_eq!(BLACK_ROOK.color(), Color::Black);
        assert_eq!(BLACK_QUEEN.color(), Color::Black);
        assert_eq!(BLACK_KING.color(), Color::Black);
    }

    #[test]
    fn piece_kind() {
        assert_eq!(VOID.kind(), kinds::UNKNOWN);
        assert_eq!(WHITE_PAWN.kind(), kinds::PAWN);
        assert_eq!(WHITE_KNIGHT.kind(), kinds::KNIGHT);
        assert_eq!(WHITE_BISHOP.kind(), kinds::BISHOP);
        assert_eq!(WHITE_ROOK.kind(), kinds::ROOK);
        assert_eq!(WHITE_QUEEN.kind(), kinds::QUEEN);
        assert_eq!(WHITE_KING.kind(), kinds::KING);
        assert_eq!(BLACK_PAWN.kind(), kinds::PAWN);
        assert_eq!(BLACK_KNIGHT.kind(), kinds::KNIGHT);
        assert_eq!(BLACK_BISHOP.kind(), kinds::BISHOP);
        assert_eq!(BLACK_ROOK.kind(), kinds::ROOK);
        assert_eq!(BLACK_QUEEN.kind(), kinds::QUEEN);
        assert_eq!(BLACK_KING.kind(), kinds::KING);
    }

    #[test]
    fn piece_fmt() {
        assert_eq!(format!("{}", VOID), "Void");
        assert_eq!(format!("{}", WHITE_PAWN), "White-pawn");
        assert_eq!(format!("{}", WHITE_KNIGHT), "White-knight");
        assert_eq!(format!("{}", WHITE_BISHOP), "White-bishop");
        assert_eq!(format!("{}", WHITE_ROOK), "White-rook");
        assert_eq!(format!("{}", WHITE_QUEEN), "White-queen");
        assert_eq!(format!("{}", WHITE_KING), "White-king");
        assert_eq!(format!("{}", BLACK_PAWN), "Black-pawn");
        assert_eq!(format!("{}", BLACK_KNIGHT), "Black-knight");
        assert_eq!(format!("{}", BLACK_BISHOP), "Black-bishop");
        assert_eq!(format!("{}", BLACK_ROOK), "Black-rook");
        assert_eq!(format!("{}", BLACK_QUEEN), "Black-queen");
        assert_eq!(format!("{}", BLACK_KING), "Black-king");
    }

    #[test]
    fn all_pieces() {
        let all = All.into_iter().collect::<Vec<_>>();
        assert_eq!(all.len(), 12);
        assert_eq!(all[0], WHITE_PAWN);
        assert_eq!(all[11], BLACK_KING);
    }
}
