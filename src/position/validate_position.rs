#![allow(dead_code, collapsible_if)]
#![allow(trivial_casts, trivial_numeric_casts)]

use super::*;

bitflags! {
    pub flags Assessment: u32 {
        const VALID = 0 ,
        const HAS_NO_WHITE_KING = 1 << 0,
        const HAS_MORE_THAN_ONE_WHITE_KING = 1 << 1,
        const HAS_NO_BLACK_KING = 1 << 2,
        const HAS_MORE_THAN_ONE_BLACK_KING = 1 << 3,
        const WHITE_PAWNS_ON_PROMOTION_RANK = 1 << 4,
        const BLACK_PAWNS_ON_PROMOTION_RANK = 1 << 5,
        const EN_PASSANT_WITHOUT_PAWN = 1 << 6,
        const EN_PASSANT_SQUARE_OCCUPIED = 1 << 7,
        const CASTLING_WITHOUT_ROOK_A1 = 1 << 8,
        const CASTLING_WITHOUT_ROOK_H1 = 1 << 9,
        const CASTLING_WITHOUT_ROOK_A8 = 1 << 10,
        const CASTLING_WITHOUT_ROOK_H8 = 1 << 11,
        const CASTLING_WITHOUT_KING_E1 = 1 << 12,
        const CASTLING_WITHOUT_KING_E8 = 1 << 13,
        const WTF= 1 << 20,
    }
}

impl Position {
    fn validate(&self) -> Assessment {
        self.white_pawns_on_promotion_rank() |
            self.black_pawns_on_promotion_rank() |
            self.has_more_than_one_white_king() |
            self.has_no_white_king() |
            self.has_more_than_one_black_king() |
            self.has_no_black_king() |
            self.validate_en_passant() |
            self.validate_castling()
    }
    fn white_pawns_on_promotion_rank(&self) -> Assessment {
        if self.board.pawns::<White>().0 & _8 != EMPTY {
            WHITE_PAWNS_ON_PROMOTION_RANK
        } else {
            VALID
        }
    }
    fn black_pawns_on_promotion_rank(&self) -> Assessment {
        if self.board.pawns::<Black>().0 & _1 != EMPTY {
            BLACK_PAWNS_ON_PROMOTION_RANK
        } else {
            VALID
        }
    }
    fn has_more_than_one_white_king(&self) -> Assessment {
        if self.board.kings::<White>().0.count() > 1 {
            HAS_MORE_THAN_ONE_WHITE_KING
        } else {
            VALID
        }
    }
    fn has_no_white_king(&self) -> Assessment {
        if self.board.kings::<White>().0.count() == 0 {
            HAS_NO_WHITE_KING
        } else {
            VALID
        }
    }
    fn has_more_than_one_black_king(&self) -> Assessment {
        if self.board.kings::<Black>().0.count() > 1 {
            HAS_MORE_THAN_ONE_BLACK_KING
        } else {
            VALID
        }
    }
    fn has_no_black_king(&self) -> Assessment {
        if self.board.kings::<Black>().0.count() == 0 {
            HAS_NO_BLACK_KING
        } else {
            VALID
        }
    }
    fn validate_en_passant(&self) -> Assessment {
        use ::rank::*;
        if let Some(file) = self.en_passant {
            let pawn_rank = if self.active == Color::White { _5 } else { _4 };
            let pawn_square = Mask::from_file_rank(file, pawn_rank);
            let pawns_of_inactive = self.board.pawns_of(self.active.invert());
            if pawns_of_inactive & pawn_square == EMPTY {
                return EN_PASSANT_WITHOUT_PAWN
            }
            let target_rank = if self.active == Color::White { _6 } else { _3 };
            let target_square = Mask::from_file_rank(file, target_rank);
            if self.board.get_piece(target_square) != VOID {
                return EN_PASSANT_SQUARE_OCCUPIED
            }
        }
        VALID
    }
    fn validate_castling(&self) -> Assessment {
        if self.available.contains(castle::WQ) {
            if self.board.rooks::<White>().0 & A1 == EMPTY {
                return CASTLING_WITHOUT_ROOK_A1
            }
        }
        if self.available.contains(castle::WK) {
            if self.board.rooks::<White>().0 & H1 == EMPTY {
                return CASTLING_WITHOUT_ROOK_H1
            }
        }
        if self.available.contains(castle::BQ) {
            if self.board.rooks::<Black>().0 & A8 == EMPTY {
                return CASTLING_WITHOUT_ROOK_A8
            }
        }
        if self.available.contains(castle::BK) {
            if self.board.rooks::<Black>().0 & H8 == EMPTY {
                return CASTLING_WITHOUT_ROOK_H8
            }
        }

        if self.available.intersects(castle::W) {
            if self.board.kings::<White>().0 & E1 == EMPTY {
                return CASTLING_WITHOUT_KING_E1
            }
        }
        if self.available.intersects(castle::B) {
            if self.board.kings::<Black>().0 & E8 == EMPTY {
                return CASTLING_WITHOUT_KING_E8
            }
        }
        VALID
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_position() {
        assert_assessment(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            VALID);
    }

    #[test]
    fn has_no_white_king() {
        assert_assessment(
            "8/8/8/k7/8/8/8/8 w - - 0 1",
            HAS_NO_WHITE_KING);
    }

    #[test]
    fn two_white_kings() {
        assert_assessment(
            "8/K7/8/8/K7/8/8/k7 w - - 0 1",
            HAS_MORE_THAN_ONE_WHITE_KING);
    }

    #[test]
    fn has_no_black_king() {
        assert_assessment(
            "8/8/K7/8/8/8/8/8 w - - 0 1",
            HAS_NO_BLACK_KING);
    }

    #[test]
    fn two_black_kings() {
        assert_assessment(
            "8/k7/8/8/k7/8/K7/8 w - - 0 1",
            HAS_MORE_THAN_ONE_BLACK_KING);
    }

    #[test]
    fn white_pawns_on_promotion_rank() {
        assert_assessment(
            "P7/8/8/8/k7/8/K7/8 w - - 0 1",
            WHITE_PAWNS_ON_PROMOTION_RANK);
    }

    #[test]
    fn black_pawns_on_promotion_rank() {
        assert_assessment(
            "8/8/8/8/k7/8/K7/p7 w - - 0 1",
            BLACK_PAWNS_ON_PROMOTION_RANK);
    }

    #[test]
    fn valid_en_passant() {
        assert_assessment(
            "8/8/8/p7/8/8/K7/7k w - a 0 1",
            VALID);
    }

    #[test]
    fn white_en_passant_without_pawn() {
        assert_assessment(
            "8/8/8/8/8/8/K7/7k w - a 0 1",
            EN_PASSANT_WITHOUT_PAWN);
    }
    #[test]
    fn black_en_passant_without_pawn() {
        assert_assessment(
            "8/8/8/8/8/8/K7/7k b - e 0 1",
            EN_PASSANT_WITHOUT_PAWN);
    }
    #[test]
    fn white_en_passant_square_occupied() {
        assert_assessment(
            "8/8/n7/p7/8/8/K7/7k w - a 0 1",
            EN_PASSANT_SQUARE_OCCUPIED);
    }
    #[test]
    fn black_en_passant_square_occupied() {
        assert_assessment(
            "8/8/8/8/4P3/4n3/K7/7k b - e 0 1",
            EN_PASSANT_SQUARE_OCCUPIED);
    }

    #[test]
    fn valid_all_castling_available() {
        assert_assessment(
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            VALID);
    }
    #[test]
    fn castling_without_king_e1() {
        assert_assessment(
            "r3k2r/8/8/8/8/8/8/R2K3R w KQkq - 0 1",
            CASTLING_WITHOUT_KING_E1);
    }
    #[test]
    fn castling_without_king_e8() {
        assert_assessment(
            "r2k3r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            CASTLING_WITHOUT_KING_E8);
    }
    #[test]
    fn valid_no_castling_available_no_rooks() {
        assert_assessment(
            "4k3/8/8/8/8/8/8/4K3 w - - 0 1",
            VALID);
    }
    #[test]
    fn castling_without_rook_h8() {
        assert_assessment(
            "r3k3/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            CASTLING_WITHOUT_ROOK_H8);
    }
    #[test]
    fn castling_without_rook_a8() {
        assert_assessment(
            "4k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            CASTLING_WITHOUT_ROOK_A8);
    }
    #[test]
    fn castling_without_rook_h1() {
        assert_assessment(
            "r3k2r/8/8/8/8/8/8/R3K3 w KQkq - 0 1",
            CASTLING_WITHOUT_ROOK_H1);
    }
    #[test]
    fn castling_without_rook_a1() {
        assert_assessment(
            "r3k2r/8/8/8/8/8/8/4K2R w KQkq - 0 1",
            CASTLING_WITHOUT_ROOK_A1);
    }


    fn assert_assessment(fen: &str, expected: Assessment) {
        assert_eq!(Position::parse(fen).validate(), expected);
    }
}