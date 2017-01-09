#![allow(dead_code)]

use super::*;

#[derive(PartialEq, Debug)]
pub enum Assessment {
    Valid, HasNoWhiteKing, HasMoreThanOneWhiteKing
}

impl Position {
    fn validate(&self) -> Assessment {
        let white_kings = self.board.kings::<White>().0.count();
        if white_kings == 0 {
            Assessment::HasNoWhiteKing
        } else if white_kings > 1 {
            Assessment::HasMoreThanOneWhiteKing
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
    fn assert_assessment(fen: &str, expected :Assessment) {
        assert_eq!(Position::parse(fen).validate(), expected);
    }
}