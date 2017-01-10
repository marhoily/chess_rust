#![allow(collapsible_if)]

use super::*;
use castle;
use color::Color;
use moves::Move;

impl Position {
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
        if mv.promote != UNKNOWN {
            if self.active == Color::White {
                if piece != WHITE_PAWN {
                    return false;
                }
                if mv.to.rank() != ::rank::_7 {
                    return false;
                }
            } else {
                if piece != BLACK_PAWN {
                    return false;
                }
                if mv.to.rank() != ::rank::_2 {
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
        if self.board.pawns_of(self.active).contains(to) {
            return self.is_pseudo_legal_pawn_move(from, to);
        }

        // determine which non-pawn piece this is and check it
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moves::*;
    use square::*;

    #[test]
    fn valid_pawn_move() {
        yes(Move::new(A2, A4), "8/8/8/8/8/8/P7/8 w - - 0 1");
    }

    #[test]
    fn source_square_must_not_be_vacant() {
        no(Move::new(E2, E4), "8/8/8/8/8/8/8/8 w - - 0 1");
    }

    fn yes(m: Move, fen: &str) {
        assert_eq!(test(fen, m), true)
    }

    fn no(m: Move, fen: &str) {
        assert_eq!(test(fen, m), false)
    }

    fn test(fen: &str, m: Move) -> bool {
        Position::parse(fen).is_pseudo_legal(m)
    }
}