use super::*;
use color::Color;
use sided_mask::*;

impl Position {
    pub fn pseudo_legal_pawn_moves(&self) -> Mask {
        if self.active == Color::White {
            self.pseudo_legal_pawn_moves_of::<White>().0
        } else {
            self.pseudo_legal_pawn_moves_of::<Black>().0
        }
    }

    pub fn pseudo_legal_pawn_moves_of<S: Side>(&self) -> S::Mask {
        let pawns = self.board.pawns::<S>();
        let empty_squares = !self.board.occupation();
        let attacks = pawns.attack();
        let non_enp_captures = attacks.filter(
            self.board.occupation_gen::<S::Opposite>());
        let enp_captures = attacks.filter(self.en_passant_take_square_mask::<S>());
        let single_pushes = pawns.advance().filter(empty_squares);
        let double_pushes = single_pushes.advance()
            .filter(empty_squares & S::DOUBLE_PUSH_RANK_MASK);
        enp_captures
            .and(non_enp_captures)
            .and(single_pushes)
            .and(double_pushes)
    }

    pub fn en_passant_take_square_mask<S: Side>(&self) -> Mask {
        self.en_passant.map_or(EMPTY,
                               |file| Mask::from_file_rank(file, S::EN_PASSANT_RANK))
    }
    #[allow(unused_variables)]
    pub fn is_pseudo_legal_pawn_move(&self, from: Mask, to: Mask) -> bool {
        // captures
        // single push
        // double push
        // en-passant
        true
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use side::*;

    #[test]
    fn generate_pseudo_legal_white_pawn_moves_single_push() {
        assert_pseudo_legal_pawn_moves(
            "8/8/8/3P4/8/8/8/8 w KQkq - 0 1", D6
        );
    }

    #[test]
    fn generate_pseudo_legal_white_pawn_moves_take_to_the_left() {
        assert_pseudo_legal_pawn_moves(
            "8/8/2pp4/3P4/8/8/8/8 w KQkq - 0 1", C6
        );
    }

    #[test]
    fn generate_pseudo_legal_black_pawn_moves_take_to_the_left() {
        assert_pseudo_legal_pawn_moves(
            "8/8/3p4/2PP4/8/8/8/8 b KQkq - 0 1", C5
        );
    }

    #[test]
    fn generate_pseudo_legal_white_pawn_moves_take_to_the_right() {
        assert_pseudo_legal_pawn_moves(
            "8/8/3pp3/3P4/8/8/8/8 w KQkq - 0 1", E6
        );
    }

    #[test]
    fn generate_pseudo_legal_white_pawn_moves_en_passant() {
        assert_pseudo_legal_pawn_moves(
            "8/8/3p4/3P4/8/8/8/8 w KQkq e 0 1", E6
        );
    }

    #[test]
    fn generate_pseudo_legal_white_pawn_moves_double_push() {
        assert_pseudo_legal_pawn_moves(
            "8/8/8/8/8/8/3P4/8 w KQkq - 0 1", D3 | D4
        );
    }

    #[test]
    fn generate_pseudo_legal_black_pawn_moves_double_push() {
        assert_pseudo_legal_pawn_moves(
            "8/3p4/8/8/8/8/8/8 b KQkq - 0 1", D6 | D5
        );
    }

    #[test]
    fn generate_pseudo_legal_black_pawn_moves_double_push_blocked() {
        assert_pseudo_legal_pawn_moves(
            "8/3p4/8/3p4/8/8/8/8 b KQkq - 0 1", D6 | D4
        );
    }
    #[test]
    fn generate_pseudo_legal_black_pawn_moves_double_push_opposed() {
        assert_pseudo_legal_pawn_moves(
            "8/3p4/8/3P4/8/8/8/8 b KQkq - 0 1", D6
        );
    }
    #[test]
    fn generate_pseudo_legal_white_pawn_moves_random() {
        assert_pseudo_legal_pawn_moves(
            "N4p2/P3P2P/8/8/p1p3kp/1P4P1/Kp4p1/P6P w - - 0 1",
            E8 |F8|H8|A4|B4|C4|H4|B2|G2|H2
        );
    }
    fn assert_pseudo_legal_pawn_moves(fen: &str, expected: Mask) {
        assert_eq!(Position::parse(fen).pseudo_legal_pawn_moves(), expected);
    }
}
