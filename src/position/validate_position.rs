#![allow(dead_code)]

use super::*;

#[derive(PartialEq, Debug)]
pub enum Assessment {
    Valid,
    HasNoWhiteKing,
    HasMoreThanOneWhiteKing,
    HasNoBlackKing,
    HasMoreThanOneBlackKing,
    WhitePawnsOnPromotionRank,
    BlackPawnsOnPromotionRank,
}

impl Position {
    fn validate(&self) -> Assessment {
        let white_kings = self.board.kings::<White>().0.count();
        let black_kings = self.board.kings::<Black>().0.count();
        if white_kings == 0 {
            Assessment::HasNoWhiteKing
        } else if white_kings > 1 {
            Assessment::HasMoreThanOneWhiteKing
        } else if black_kings == 0 {
            Assessment::HasNoBlackKing
        } else if black_kings > 1 {
            Assessment::HasMoreThanOneBlackKing
        } else if self.board.pawns::<White>().0 & _8 != EMPTY {
            Assessment::WhitePawnsOnPromotionRank
        } else if self.board.pawns::<Black>().0 & _1 != EMPTY {
            Assessment::BlackPawnsOnPromotionRank
        } else {
            Assessment::Valid
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_position() {
        assert_assessment(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            Assessment::Valid);
    }

    #[test]
    fn has_no_white_king() {
        assert_assessment(
            "8/8/8/8/8/8/8/8 w - - 0 1",
            Assessment::HasNoWhiteKing);
    }

    #[test]
    fn two_white_kings() {
        assert_assessment(
            "8/K7/8/8/K7/8/8/8 w - - 0 1",
            Assessment::HasMoreThanOneWhiteKing);
    }

    #[test]
    fn has_no_black_king() {
        assert_assessment(
            "8/8/K7/8/8/8/8/8 w - - 0 1",
            Assessment::HasNoBlackKing);
    }

    #[test]
    fn two_black_kings() {
        assert_assessment(
            "8/k7/8/8/k7/8/K7/8 w - - 0 1",
            Assessment::HasMoreThanOneBlackKing);
    }

    #[test]
    fn white_pawns_on_promotion_rank() {
        assert_assessment(
            "P7/8/8/8/k7/8/K7/8 w - - 0 1",
            Assessment::WhitePawnsOnPromotionRank);
    }

    #[test]
    fn black_pawns_on_promotion_rank() {
        assert_assessment(
            "8/8/8/8/k7/8/K7/p7 w - - 0 1",
            Assessment::BlackPawnsOnPromotionRank);
    }

    fn assert_assessment(fen: &str, expected: Assessment) {
        assert_eq!(Position::parse(fen).validate(), expected);
    }
}