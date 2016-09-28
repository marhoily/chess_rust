#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use bit_board::*;
use nom::{IResult};
use nom::IResult::*;
use std::str;
use std::str::FromStr;

impl BitBoard {
    pub fn print_fen(&self) -> String{
        let mut sb = String::with_capacity(60);
        let mut file = 0;
        let mut gap = 0;
        for square in self.squares() {
            if file == 8 {
                sb.push('/');
                file = 0;
            }
            file += 1;
            if square == EMPTY {
                gap += 1;
            }
            else {
                if gap > 0 {
                    sb.push_str(format!("{}", gap).as_str());
                    gap = 0;
                }
                sb.push(square.as_char())
            }
            if file == 8 && gap > 0 {
                sb.push_str(format!("{}", gap).as_str());
                gap = 0;
            }
        }
        sb
    }
}


pub fn board(input: &[u8]) -> IResult<&[u8], BitBoard> {
    use nom::{Err, ErrorKind};
    let mut result = BitBoard::new();
    let mut square = SquareExp::new(1);
    let mut rank = 0;
    let mut file = 0; 
    let mut just_had_gap = false;
    let mut consumed = 0;
    for &e in input {
        match consume(e as char) {
            Symbol::Id(p) => {

            }
            Symbol::Gap(size) => {

            }
            Symbol::Slash => {

            }
            Symbol::Other => {
                return Error(Err::Code(ErrorKind::Custom(0)))
            }
        }
        consumed += 1;
    }    
    Done(&input[consumed..], result)
}

enum Symbol {
    Id(Piece),
    Gap(u8),
    Slash,
    Other
}

fn consume(c : char) -> Symbol {
    match c {
        'P' => Symbol::Id(WHITE_PAWN),
        'N' => Symbol::Id(WHITE_KNIGHT),
        'B' => Symbol::Id(WHITE_BISHOP),
        'R' => Symbol::Id(WHITE_ROOK),
        'Q' => Symbol::Id(WHITE_QUEEN),
        'K' => Symbol::Id(WHITE_KING),
        'p' => Symbol::Id(BLACK_PAWN),
        'n' => Symbol::Id(BLACK_KNIGHT),
        'b' => Symbol::Id(BLACK_BISHOP),
        'r' => Symbol::Id(BLACK_ROOK),
        'q' => Symbol::Id(BLACK_QUEEN),
        'k' => Symbol::Id(BLACK_KING),

        '1' => Symbol::Gap(1),
        '2' => Symbol::Gap(2),
        '3' => Symbol::Gap(3),
        '4' => Symbol::Gap(4),
        '5' => Symbol::Gap(5),
        '6' => Symbol::Gap(6),
        '7' => Symbol::Gap(7),
        '8' => Symbol::Gap(8),

        '/' => Symbol::Slash,

        _ => Symbol::Other,
    }
}
 

#[cfg(test)]
mod test {
    use bit_board::*;
    use nom::IResult;
    use nom::IResult::*;
    use super::board;

    #[test]
    fn print_fen() {
        let mut b = BitBoard::new();
        let a7 = Square64::new(0+8).to_exp();
        let e4 = Square64::new(4+32).to_exp();
        b.set_piece(a7, BLACK_PAWN);
        b.set_piece(e4, WHITE_QUEEN);
        assert_eq!(b.print_fen(), "8/p7/8/8/4Q3/8/8/8");
    }
    #[test]
    fn parse_fen() {
        check("8/p7/8/8/4Q3/8/8/8");
    }
    fn check(fen : &str){
        assert_eq!(
            board(fen.as_bytes()).unwrap().1.print_fen(),
            fen);

    }
}
