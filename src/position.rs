#![allow(dead_code)]
#![allow(collapsible_if)]

use castle;
use castle::Castle;
use bit_board::BitBoard;
use color::Color;
use file::File;
use bit_board::fen;
use self::wrappers::*;
use piece::pieces::*;
use moves::Move;
use kind::kinds;
use mask::Mask;

#[derive(Eq, Debug, Copy, Clone, PartialEq)]
pub struct Position {
    board: BitBoard,
    active: Color,
    available: Castle,
    en_passant: Option<File>,
}
impl Position {
    pub fn parse(input: &str) -> Self {
        parse_position(input.as_bytes()).unwrap().1
    }
    #[allow(unused_variables)]
    pub fn is_pseudo_legal_pawn_move(&self, from: Mask, to : Mask) -> bool {
        // captures
        // single push
        // double push
        // en-passant
        true
    }
    pub fn is_pseudo_legal(&self, mv: Move) -> bool {
        // Source square must not be vacant.
        let from = mv.from.mask();
        let piece = self.board.get_piece(from);
        if piece == VOID {
            return false;
        }
        // Check turn.
        if !self.board.occupation_of(self.active).contains(from) {
            return false;
        }

        //  Only pawns can promote and only on the back-rank.
        if mv.promote != kinds::UNKNOWN {
            if self.active == Color::White {
                if piece != WHITE_PAWN {
                    return false;
                }
                if mv.to.rank() != ::rank::ranks::_7 {
                    return false;
                }
            } else {
                if piece != BLACK_PAWN {
                    return false;
                }
                if mv.to.rank() != ::rank::ranks::_2 {
                    return false;
                }

            }
        }
        if mv.castle != castle::NONE {
            if self.available.contains(mv.castle & self.active.castle()) {
                return true;
            }
        }

        // Destination square can not be occupied.
        let to = mv.to.mask();
        // Do squares occupied by active side contain `to`?
        if self.board.occupation_of(self.active).contains(to) {
            return false;
        }

        // Handle pawn pushes
        if self.board.pawns(self.active).contains(to) {
            return self.is_pseudo_legal_pawn_move(from, to);
        }

        self.board.attacks(self.active).contains(to)
    }
}
impl ::std::fmt::Display for Position {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let r = self.en_passant.map_or('-', |x| x.char());
        write!(f, "{} {} {} {}", self.board, self.active, self.available, r)
    }
}
#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum PositionError {
    Board(fen::ParsingError),
    Active(u32),
    Available(castle::ParsingError),
    EnPassant(u32),
    Whitespace,
}

mod wrappers {
    use super::*;
    use castle::Castle;
    use bit_board::BitBoard;
    use color::Color;
    use file::File;
    use nom::Err::Position as P;
    use nom::ErrorKind::Custom as C;
    use super::PositionError::*;
    type R<'a, T, X> = ::nom::IResult<&'a [u8], T, X>;

    pub fn parse_bit_board(input: &[u8]) -> R<BitBoard, PositionError> {
        ::bit_board::fen::parse_bit_board(input).map_err(|err| {
            match err {
                P(C(pe), x) => P(C(Board(pe)), x),
                _ => panic!("parse_bit_board"),
            }
        })
    }

    pub fn parse_color(input: &[u8]) -> R<Color, PositionError> {
        ::color::parse_color(input).map_err(|err| {
            match err {
                P(C(pe), x) => P(C(Active(pe)), x),
                _ => panic!("parse_color"),
            }
        })
    }

    pub fn parse_castle(input: &[u8]) -> R<Castle, PositionError> {
        ::castle::parse_castle(input).map_err(|err| {
            match err {
                P(C(pe), x) => P(C(Available(pe)), x),
                _ => panic!("parse_castle"),
            }
        })
    }

    pub fn parse_file(input: &[u8]) -> R<File, PositionError> {
        ::file::parse_file(input).map_err(|err| {
            match err {
                P(C(pe), x) => P(C(EnPassant(pe)), x),
                _ => panic!("parse_file"),
            }
        })
    }

    named!(ws_inner(&[u8]) -> char, char!(' '));
    pub fn ws(input: &[u8]) -> R<char, PositionError> {
        ws_inner(input).map_err(|err| {
            match err {
                P(_, x) => P(C(Whitespace), x),
                _ => panic!("ws"),
            }
        })
    }
}

// "8/8/8/8/8/8/8/8 w KQkq - 0 1"
named!(pub parse_position<&[u8], Position, PositionError>,
    chain!(
        squares: parse_bit_board ~ ws ~
        side: parse_color ~ ws ~
        castle: parse_castle ~ ws ~
        file: parse_file,
        || Position {
                board: squares,
                active: side,
                available: castle,
                en_passant: Some(file)
        }));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_fen() {
        assert_eq!(format!("{}",
                Position::parse("8/8/8/8/8/8/8/8 w KQkq e 0 1")),
                "8/8/8/8/8/8/8/8 w KQkq e");
    }
}
