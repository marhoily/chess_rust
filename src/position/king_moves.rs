use super::*;
use color::Color;
use sided_mask::*;

impl Position {
    pub fn pseudo_legal_king_moves(&self) -> Mask {
        if self.active == Color::White {
            self.pseudo_legal_king_moves_of::<White>()
        } else {
            self.pseudo_legal_king_moves_of::<Black>()
        }
    }

    pub fn pseudo_legal_king_moves_of<S: Side>(&self) -> Mask {
        self.board.kings::<S>().mask().king_attacks() &
            !self.board.occupation_gen::<S>().mask()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use mask::masks::*;

    #[test]
    fn pseudo_legal_white_king_moves_center() {
        assert_pseudo_legal_king_moves(
            "8/8/8/4N3/4Kp2/8/8/8 w - - 0 1",
            D3 | D4| D5 |F5 | F4 |F3 | E3
        );
    }

    fn assert_pseudo_legal_king_moves(fen: &str, expected: Mask) {
        assert_eq!(Position::parse(fen).pseudo_legal_king_moves(), expected);
    }
}