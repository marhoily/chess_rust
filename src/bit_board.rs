#![allow(dead_code)]

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

struct File(i8);

struct Rank(i8);

// Note that index 0 corresponds to a8, and NOT a1!
// Indexes read left to right, top to bottom!
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Square64(i8);

impl Square64 {
    pub fn new(square_number: i8) -> Self {
        Square64(square_number)
    }
    pub fn to_exp(&self) -> SquareExp {
        SquareExp(1 << self.0)
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct SquareExp(u64);

impl SquareExp {
    pub fn new(exp: u64) -> Self {
        SquareExp(exp)
    }
    pub fn next(&mut self) {
        self.0 <<= 1;
    }
    pub fn forward(&mut self, count : u8) {
        self.0 <<= count;
    }
}


#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Piece(i32);

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct PieceType(i32);

impl Debug for PieceType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0 {
            -1 => write!(f, "unknown"),
            0 => write!(f, "pawn"),
            1 => write!(f, "knight"),
            2 => write!(f, "bishop"),
            3 => write!(f, "rook"),
            4 => write!(f, "queen"),
            5 => write!(f, "king"),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

const PIECE_TYPES_COUNT: i32 = 6;
const PIECES_COUNT: usize = 12;
static PIECE_CHARS: &'static [u8; 12] = b"PNBRQKpnbrqk";

impl Piece {
    pub fn get_color(&self) -> Color {
        if self.0 >= PIECE_TYPES_COUNT {
            Color::Black
        } else {
            Color::White
        }
    }
    pub fn get_type(&self) -> PieceType {
        PieceType(self.0 % PIECE_TYPES_COUNT)
    }
    pub fn as_char(&self) -> char {
        PIECE_CHARS[self.0 as usize] as char
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if *self == EMPTY {
            write!(f, "Empty")
        } else {
            write!(f, "{:?}-{:?}", self.get_color(), self.get_type())
        }
    }
}

pub const PAWN: PieceType = PieceType(0);
pub const KNIGHT: PieceType = PieceType(1);
pub const BISHOP: PieceType = PieceType(2);
pub const ROOK: PieceType = PieceType(3);
pub const QUEEN: PieceType = PieceType(4);
pub const KING: PieceType = PieceType(5);
pub const UNKNOWN: PieceType = PieceType(-1);

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
pub const EMPTY: Piece = Piece(-1);

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PieceTypeBits(u64);

#[derive(Debug, PartialEq)]
pub struct BitBoard([PieceTypeBits; PIECES_COUNT]);

impl PieceTypeBits {
    fn test(self, square: SquareExp) -> bool {
        self.0 & square.0 != 0
    }
    fn set(&mut self, square: SquareExp) {
        self.0 |= square.0
    }
    fn count(self) -> u32 {
        self.0.count_ones()
    }
}

struct AllPieces;

impl IntoIterator for AllPieces {
    type Item = Piece;
    type IntoIter = PieceIter;

    fn into_iter(self) -> Self::IntoIter {
        PieceIter(0)
    }
}

struct PieceIter(i32);

impl Iterator for PieceIter {
    type Item = Piece;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < 12 {
            let result = Piece(self.0);
            self.0 += 1;
            Some(result)
        } else {
            None
        }
    }
}

pub struct AllSquaresExp;

impl IntoIterator for AllSquaresExp {
    type Item = SquareExp;
    type IntoIter = SquareExpIter;

    fn into_iter(self) -> Self::IntoIter {
        SquareExpIter::new()
    }
}

pub struct SquareExpIter(u64);
impl SquareExpIter {
    pub fn new() -> Self{
        SquareExpIter(1)
    }
}

impl Iterator for SquareExpIter {
    type Item = SquareExp;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let result = SquareExp(self.0);
            self.0 <<= 1;
            Some(result)
        }
    }
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard([PieceTypeBits(0); PIECES_COUNT])
    }
    pub fn for_piece(&self, piece: Piece) -> PieceTypeBits {
        self.0[piece.0 as usize]
    }
    pub fn check_square(&self, square: SquareExp) -> Piece {
        for piece in AllPieces {
            if self.for_piece(piece).test(square) {
                return piece;
            }
        }
        EMPTY
    }
    pub fn set_piece(&mut self, square: SquareExp, piece: Piece) {
        self.0[piece.0 as usize].0 |= square.0;
    }
    pub fn get_piece(&self, square: SquareExp) -> Piece {
        for probe in AllPieces {
            if self.for_piece(probe).test(square) {
                return probe;
            }
        }
        EMPTY
    }
    pub fn squares<'a>(&'a self) -> SquareIter<'a> {
        SquareIter {
            board: &self,
            square_iter: SquareExpIter::new(),
        }
    }
}

pub struct SquareIter<'a> {
    board: &'a BitBoard,
    square_iter: SquareExpIter,
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        self.square_iter.next().map(|square| self.board.get_piece(square))
    }
}
bitflags! {
    pub flags Castling: u8 {
        //const None = 0,
        const Q = WQ.bits | BQ.bits,
        const K = WK.bits | BK.bits,
        const W = WQ.bits | WK.bits,
        const B = BQ.bits | BK.bits,
        const WQ = 1 << 0,
        const WK = 1 << 2,
        const BQ = 1 << 3,
        const BK = 1 << 4,
        const ALL = Q.bits | K.bits,
    }
}

enum MoveAnnotations {
    None,
    Promotion,
    Capture,
    EnPassant,
    DoublePush,
}

enum Warnings {
    None,
    MissingPromotionHint,
    SparePromotion,
}

enum Errors {
    None,

    MoveToCheck,
    FromEmptyCell,
    ToOccupiedCell,
    WrongSideToMove,

    CastleFromCheck,
    CastleThroughCheck,
    HasNoCastling,

    HasNoEnPassant,

    DoesNotMoveThisWay,
    DoesNotCaptureThisWay,
    OnlyCapturesThisWay,
    JumpsOverPieces,
}

pub struct Move(u16);

const MOVE_FROM_MASK: u16 = 0b0000_0000_0000_1111;
const MOVE_TO_MASK: u16 = 0b0000_0000_1111_0000;
const MOVE_PROMOTE_TO_MASK: u16 = 0b0000_0011_0000_0000;

impl Move {
    pub fn new(from: Square64, to: Square64) -> Self {
        Move((from.0 as u16) | ((to.0 as u16) << 4))
    }
    pub fn wiht_promotion(from: Square64, to: Square64, promote_to: PieceType) -> Self {
        Move((from.0 as u16) | ((to.0 as u16) << 4) | ((promote_to.0 as u16) << 8))
    }
    pub fn get_from(self) -> Square64 {
        Square64(((self.0 as u16) & MOVE_FROM_MASK) as i8)
    }
    pub fn get_to(self) -> Square64 {
        Square64((((self.0 as u16) & MOVE_TO_MASK) >> 4) as i8)
    }
    pub fn get_promote_to(self) -> PieceType {
        PieceType((((self.0 as u16) & MOVE_PROMOTE_TO_MASK) >> 8) as i32)
    }
}

struct Position {
    board: BitBoard,
    active: Color,
    available: Castling,
    en_passant: Option<File>,
}

#[cfg(test)]
mod test {
    use super::BitBoard;
    use super::SquareExp;
    use super::Color;

    #[test]
    fn piece_get_color() {
        assert_eq!(super::WHITE_PAWN.get_color(), Color::White);
        assert_eq!(super::WHITE_KNIGHT.get_color(), Color::White);
        assert_eq!(super::WHITE_BISHOP.get_color(), Color::White);
        assert_eq!(super::WHITE_ROOK.get_color(), Color::White);
        assert_eq!(super::WHITE_QUEEN.get_color(), Color::White);
        assert_eq!(super::WHITE_KING.get_color(), Color::White);
        assert_eq!(super::BLACK_PAWN.get_color(), Color::Black);
        assert_eq!(super::BLACK_KNIGHT.get_color(), Color::Black);
        assert_eq!(super::BLACK_BISHOP.get_color(), Color::Black);
        assert_eq!(super::BLACK_ROOK.get_color(), Color::Black);
        assert_eq!(super::BLACK_QUEEN.get_color(), Color::Black);
        assert_eq!(super::BLACK_KING.get_color(), Color::Black);
    }

    #[test]
    fn piece_get_type() {
        assert_eq!(super::EMPTY.get_type(), super::UNKNOWN);
        assert_eq!(super::WHITE_PAWN.get_type(), super::PAWN);
        assert_eq!(super::WHITE_KNIGHT.get_type(), super::KNIGHT);
        assert_eq!(super::WHITE_BISHOP.get_type(), super::BISHOP);
        assert_eq!(super::WHITE_ROOK.get_type(), super::ROOK);
        assert_eq!(super::WHITE_QUEEN.get_type(), super::QUEEN);
        assert_eq!(super::WHITE_KING.get_type(), super::KING);
        assert_eq!(super::BLACK_PAWN.get_type(), super::PAWN);
        assert_eq!(super::BLACK_KNIGHT.get_type(), super::KNIGHT);
        assert_eq!(super::BLACK_BISHOP.get_type(), super::BISHOP);
        assert_eq!(super::BLACK_ROOK.get_type(), super::ROOK);
        assert_eq!(super::BLACK_QUEEN.get_type(), super::QUEEN);
        assert_eq!(super::BLACK_KING.get_type(), super::KING);
    }

    #[test]
    fn piece_type_fmt() {
        assert_eq!(format!("{:?}", super::UNKNOWN), "unknown");
        assert_eq!(format!("{:?}", super::PAWN), "pawn");
        assert_eq!(format!("{:?}", super::KNIGHT), "knight");
        assert_eq!(format!("{:?}", super::BISHOP), "bishop");
        assert_eq!(format!("{:?}", super::ROOK), "rook");
        assert_eq!(format!("{:?}", super::QUEEN), "queen");
        assert_eq!(format!("{:?}", super::KING), "king");
    }

    #[test]
    fn piece_fmt() {
        assert_eq!(format!("{:?}", super::EMPTY), "Empty");
        assert_eq!(format!("{:?}", super::WHITE_PAWN), "White-pawn");
        assert_eq!(format!("{:?}", super::WHITE_KNIGHT), "White-knight");
        assert_eq!(format!("{:?}", super::WHITE_BISHOP), "White-bishop");
        assert_eq!(format!("{:?}", super::WHITE_ROOK), "White-rook");
        assert_eq!(format!("{:?}", super::WHITE_QUEEN), "White-queen");
        assert_eq!(format!("{:?}", super::WHITE_KING), "White-king");
        assert_eq!(format!("{:?}", super::BLACK_PAWN), "Black-pawn");
        assert_eq!(format!("{:?}", super::BLACK_KNIGHT), "Black-knight");
        assert_eq!(format!("{:?}", super::BLACK_BISHOP), "Black-bishop");
        assert_eq!(format!("{:?}", super::BLACK_ROOK), "Black-rook");
        assert_eq!(format!("{:?}", super::BLACK_QUEEN), "Black-queen");
        assert_eq!(format!("{:?}", super::BLACK_KING), "Black-king");
    }

    #[test]
    fn check_square() {
        let mut b = BitBoard::new();
        b.set_piece(SquareExp(0b0001), super::BLACK_ROOK);
        b.set_piece(SquareExp(0b0100), super::BLACK_ROOK);
        assert_eq!(b.check_square(SquareExp(0b0001)), super::BLACK_ROOK);
        assert_eq!(b.check_square(SquareExp(0b0001)), super::BLACK_ROOK);
    }

    #[test]
    fn all_pieces() {
        use std::iter::*;
        use super::Piece;

        let all = super::AllPieces.into_iter()
            .collect::<Vec<Piece>>();
        assert_eq!(all.len(), 12);
        assert_eq!(all[0], super::WHITE_PAWN);
        assert_eq!(all[11], super::BLACK_KING);
    }

    #[test]
    fn all_squares_exp() {
        use std::iter::*;

        let all = super::AllSquaresExp.into_iter()
            .collect::<Vec<SquareExp>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], SquareExp(1));
        assert_eq!(all[63], SquareExp(1 << 63));
    }
    #[test]
    fn bit_board_squares() {
        use std::iter::*;

        let mut b = BitBoard::new();
        b.set_piece(SquareExp(0b0001), super::BLACK_ROOK);

        let all = b.squares()
            .collect::<Vec<super::Piece>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], super::BLACK_ROOK);
        assert_eq!(all[63], super::EMPTY);
    }
}
