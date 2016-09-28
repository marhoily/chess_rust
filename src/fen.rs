#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use bit_board::*;
use std::string::ToString;
use std::fmt::Display;
use std::i32;

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


use nom::{IResult, digit};
use nom::IResult::*;
use std::str;
use std::str::FromStr;


#[derive(Debug, PartialEq)]
pub enum FenItem {
    Gap(u8),
    Pce(Piece),
}

named!(item<FenItem>,
    alt!(
        chain!(char!('P'), || FenItem::Pce(WHITE_PAWN)) |
        chain!(char!('N'), || FenItem::Pce(WHITE_KNIGHT)) |
        chain!(char!('B'), || FenItem::Pce(WHITE_BISHOP)) |
        chain!(char!('R'), || FenItem::Pce(WHITE_ROOK)) |
        chain!(char!('Q'), || FenItem::Pce(WHITE_QUEEN)) |
        chain!(char!('K'), || FenItem::Pce(WHITE_KING)) |
        chain!(char!('p'), || FenItem::Pce(BLACK_PAWN)) |
        chain!(char!('n'), || FenItem::Pce(BLACK_KNIGHT)) |
        chain!(char!('b'), || FenItem::Pce(BLACK_BISHOP)) |
        chain!(char!('r'), || FenItem::Pce(BLACK_ROOK)) |
        chain!(char!('q'), || FenItem::Pce(WHITE_QUEEN)) |
        chain!(char!('k'), || FenItem::Pce(BLACK_KING)) |

        chain!(char!('1'), || FenItem::Gap(1)) |
        chain!(char!('2'), || FenItem::Gap(2)) |
        chain!(char!('3'), || FenItem::Gap(3)) |
        chain!(char!('4'), || FenItem::Gap(4)) |
        chain!(char!('5'), || FenItem::Gap(5)) |
        chain!(char!('6'), || FenItem::Gap(6)) |
        chain!(char!('7'), || FenItem::Gap(7)) |
        chain!(char!('8'), || FenItem::Gap(8))
    )
);

#[cfg(test)]
mod test {
    use bit_board::*;
    use nom::{IResult, digit};
    use nom::IResult::*;
    use super::FenItem;
    use super::item;

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
        assert_eq!(item(b"Q"),
            IResult::Done(&b""[..], FenItem::Pce(WHITE_QUEEN)));

        assert_eq!(item(b"p"),
            IResult::Done(&b""[..], FenItem::Pce(BLACK_PAWN)));

        assert_eq!(item(b"1"),
            IResult::Done(&b""[..], FenItem::Gap(1)));
    }
}
