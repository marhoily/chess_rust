use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use geometry::*;
use piece_type::{PieceType, piece_types};

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct Piece(u8);

impl Piece {
    pub fn new(bits: u8) -> Self {
        Piece(bits)
    }
    pub fn bits(self) -> u8 {
        self.0
    }
    pub fn color(self) -> Color {
        if self.0 >= piece_types::COUNT {
            Color::Black
        } else {
            Color::White
        }
    }
    pub fn get_type(&self) -> PieceType {
        if *self == pieces::EMPTY {
            piece_types::UNKNOWN
        } else {
            PieceType::new(self.bits() % piece_types::COUNT)
        }
    }
    pub fn as_char(&self) -> char {
        debug_assert!(*self != pieces::EMPTY, "attempt to pieces::EMPTY.as_char()");
        SYMBOLS[self.0 as usize] as char
    }
    pub fn parse(input: &str) -> Self {
        parse_piece(input.as_bytes()).unwrap().1
    }
}

named!(parse_piece(&[u8]) -> Piece,
    map!(is_a!(SYMBOLS), |c| {
        Piece(SYMBOLS.iter().position(|x| {
            *x == (c as &[u8])[0]}).unwrap() as u8)}));

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self == pieces::EMPTY {
            write!(f, "Empty")
        } else {
            write!(f, "{:?}-{}", self.color(), self.get_type())
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
    pub const EMPTY: Piece = Piece(16);

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
    use piece_type::piece_types;

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
    fn piece_get_type() {
        assert_eq!(EMPTY.get_type(), piece_types::UNKNOWN);
        assert_eq!(WHITE_PAWN.get_type(), piece_types::PAWN);
        assert_eq!(WHITE_KNIGHT.get_type(), piece_types::KNIGHT);
        assert_eq!(WHITE_BISHOP.get_type(), piece_types::BISHOP);
        assert_eq!(WHITE_ROOK.get_type(), piece_types::ROOK);
        assert_eq!(WHITE_QUEEN.get_type(), piece_types::QUEEN);
        assert_eq!(WHITE_KING.get_type(), piece_types::KING);
        assert_eq!(BLACK_PAWN.get_type(), piece_types::PAWN);
        assert_eq!(BLACK_KNIGHT.get_type(), piece_types::KNIGHT);
        assert_eq!(BLACK_BISHOP.get_type(), piece_types::BISHOP);
        assert_eq!(BLACK_ROOK.get_type(), piece_types::ROOK);
        assert_eq!(BLACK_QUEEN.get_type(), piece_types::QUEEN);
        assert_eq!(BLACK_KING.get_type(), piece_types::KING);
    }

    #[test]
    fn piece_fmt() {
        assert_eq!(format!("{}", EMPTY), "Empty");
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
