#![allow(dead_code)]

use bit_board::BitBoard;
use colored_squares::{File, Color};

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

struct Position {
    board: BitBoard,
    active: Color,
    available: Castling,
    en_passant: Option<File>,
}