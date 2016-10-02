use geometry::*;
use piece::*;
use mask::{Mask, masks};
use bit_board::*;
use nom::IResult;
use nom::IResult::*;
use std::str;

impl BitBoard {
    pub fn print_fen(&self) -> String {
        let mut sb = String::with_capacity(60);
        let mut file = 0;
        let mut gap = 0;
        for square in self.squares() {
            if file == 8 {
                sb.push('/');
                file = 0;
            }
            file += 1;
            if square == pieces::VOID {
                gap += 1;
            } else {
                if gap > 0 {
                    sb.push_str(format!("{}", gap).as_str());
                    gap = 0;
                }
                sb.push(square.char())
            }
            if file == 8 && gap > 0 {
                sb.push_str(format!("{}", gap).as_str());
                gap = 0;
            }
        }
        sb
    }
}

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum ParsingError {
    DoubleGap,
    RankIsTooLong,
    GapIsTooBig,
    RankIsTooShort,
    UnrecognizedToken,
}

pub fn parse_bit_borad(input: &[u8]) -> IResult<&[u8], BitBoard, ParsingError> {
    use self::ParsingError::*;
    use nom::Needed::Unknown;
    use nom::Err::Position;
    use nom::ErrorKind::Custom;

    let mut result = BitBoard::new();
    let mut square = Mask::from(squares::FIRST);
    let mut file = 0;
    let mut just_had_gap = false;
    let mut consumed = 0;
    for &e in input {
        let token = if square != masks::EMPTY {
            consume(e as char)
        } else {
            Token::Slash
        };
        match token {
            Token::Piece(p) => {
                if file > 7 {
                    return Error(Position(Custom(RankIsTooLong), &input[consumed..]));
                }

                result.set_piece(square, p);
                square <<= 1;
                just_had_gap = false;
                file += 1;
            }
            Token::Gap(size) => {
                if just_had_gap {
                    return Error(Position(Custom(DoubleGap), &input[consumed..]));
                }
                square <<= size;
                just_had_gap = true;
                file += size;

                if file > 8 {
                    return Error(Position(Custom(GapIsTooBig), &input[consumed..]));
                }
            }
            Token::Slash => {
                if file < 8 {
                    return Error(Position(Custom(RankIsTooShort), &input[consumed..]));
                }
                file = 0;
                just_had_gap = false;
            }
            Token::Other => return Error(Position(Custom(UnrecognizedToken), &input[consumed..])),
        }
        consumed += 1;
    }
    if square == masks::EMPTY {
        Done(&input[consumed..], result)
    } else {
        Incomplete(Unknown)
    }
}

enum Token {
    Piece(Piece),
    Gap(u8),
    Slash,
    Other,
}

fn consume(c: char) -> Token {
    use piece::pieces::*;
    match c {
        'P' => Token::Piece(WHITE_PAWN),
        'N' => Token::Piece(WHITE_KNIGHT),
        'B' => Token::Piece(WHITE_BISHOP),
        'R' => Token::Piece(WHITE_ROOK),
        'Q' => Token::Piece(WHITE_QUEEN),
        'K' => Token::Piece(WHITE_KING),
        'p' => Token::Piece(BLACK_PAWN),
        'n' => Token::Piece(BLACK_KNIGHT),
        'b' => Token::Piece(BLACK_BISHOP),
        'r' => Token::Piece(BLACK_ROOK),
        'q' => Token::Piece(BLACK_QUEEN),
        'k' => Token::Piece(BLACK_KING),

        '1' => Token::Gap(1),
        '2' => Token::Gap(2),
        '3' => Token::Gap(3),
        '4' => Token::Gap(4),
        '5' => Token::Gap(5),
        '6' => Token::Gap(6),
        '7' => Token::Gap(7),
        '8' => Token::Gap(8),

        '/' => Token::Slash,

        _ => Token::Other,
    }
}


#[cfg(test)]
mod test {
    use geometry::squares;
    use bit_board::BitBoard;
    use piece::pieces;
    use nom::{Err, ErrorKind, Needed};
    use super::parse_bit_borad;

    #[test]
    fn print_fen() {
        let mut b = BitBoard::new();
        let a7 = squares::A7.mask();
        let e4 = squares::E4.mask();
        b.set_piece(a7, pieces::BLACK_PAWN);
        b.set_piece(e4, pieces::WHITE_QUEEN);
        assert_eq!(b.print_fen(), "8/p7/8/8/4Q3/8/8/8");
    }

    #[test]
    fn correct_fen() {
        check("8/p7/8/8/4Q3/8/8/8");
        check("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }

    #[test]
    fn double_gap() {
        expect_error("pp51whatewer", super::ParsingError::DoubleGap, 3);
    }

    #[test]
    fn rank_is_too_long() {
        expect_error("p7Qwhatewer", super::ParsingError::RankIsTooLong, 2);
    }

    #[test]
    fn gap_is_too_big() {
        expect_error("pp7whatewer", super::ParsingError::GapIsTooBig, 2);
    }

    #[test]
    fn rank_is_too_short() {
        expect_error("p6/whatewer", super::ParsingError::RankIsTooShort, 2);
        expect_error("6p/whatewer", super::ParsingError::RankIsTooShort, 2);
    }

    #[test]
    fn incomplete() {
        expect_incomplete("8");
        expect_incomplete("8/p7/8/8/");
        expect_incomplete("8/p7/8/8/4");
        expect_incomplete("8/p7/8/8/4Q3/8/8/7");
        expect_incomplete("8/p7/8/8/4Q3/8/8/7");
        expect_incomplete("8/p7/8/8/4Q3/8/8/6p");
        expect_incomplete("8/p7/8/8/4Q3/8/8/6p");
    }

    #[test]
    fn unrecognized_token() {
        expect_error("p7/whatewer", super::ParsingError::UnrecognizedToken, 3);
    }

    #[test]
    fn extra_symbols() {
        check_extra("8/p7/8/8/4Q3/8/8/81132", 18);
        check_extra("8/p7/8/8/4Q3/8/8/8p", 18);
        check_extra("8/p7/8/8/4Q3/8/8/8[", 18);
        check_extra("8/p7/8/8/4Q3/8/8/8 ", 18);
    }

    fn check(fen: &str) {
        let parse = parse_bit_borad(fen.as_bytes());
        if parse.is_err() {
            panic!("{:?}", parse.unwrap_err());
        }
        assert_eq!(parse.unwrap().1.print_fen(), fen);
    }

    fn check_extra(fen: &str, expected_stop: usize) {
        let parse = parse_bit_borad(fen.as_bytes());
        if parse.is_err() {
            panic!("{:?}", parse.unwrap_err());
        }
        assert_eq!(parse.unwrap().1.print_fen(), fen[..expected_stop]);
    }

    fn expect_incomplete(fen: &str) {
        assert_eq!(parse_bit_borad(fen.as_bytes()).unwrap_inc(),
                   Needed::Unknown);
    }

    fn expect_error(fen: &str, expected_error: super::ParsingError, expected_position: usize) {
        let input = fen.as_bytes();
        assert_eq!(parse_bit_borad(input).unwrap_err(),
                   Err::Position(ErrorKind::Custom(expected_error),
                                 &input[expected_position..]));
    }
}
