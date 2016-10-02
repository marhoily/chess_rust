#![allow(dead_code)]

use castle::Castle;
use bit_board::BitBoard;
use geometry::{File, Color};

struct Position {
    board: BitBoard,
    active: Color,
    available: Castle,
    en_passant: Option<File>,
}
