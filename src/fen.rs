#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use bit_board::BitBoard;
use bit_board::EMPTY;
use bit_board::Piece;
use bit_board::AllSquaresExp;
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

// Parser definition

use std::str;
use std::str::FromStr;

named!(parens<i64>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(i64_digit<i64>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

// We transform an integer string into a i64
// we look for a digit suite, and try to convert it.
// if either str::from_utf8 or FromStr::from_str fail,
// the parser will fail
named!(factor<i64>,
  alt!(
    i64_digit
  | parens
  )
);

// we define acc as mutable to update its value whenever a new term is found
named!(term <i64>,
  chain!(
    mut acc: factor  ~
             many0!(
               alt!(
                 tap!(mul: preceded!(tag!("*"), factor) => acc = acc * mul) |
                 tap!(div: preceded!(tag!("/"), factor) => acc = acc / div)
               )
             ),
    || { return acc }
  )
);

named!(expr <i64>,
  chain!(
    mut acc: term  ~
             many0!(
               alt!(
                 tap!(add: preceded!(tag!("+"), term) => acc = acc + add) |
                 tap!(sub: preceded!(tag!("-"), term) => acc = acc - sub)
               )
             ),
    || { return acc }
  )
);

#[cfg(test)]
mod test {
    use bit_board::*;
    use nom::{IResult, digit};
    use nom::IResult::*;
    use super::expr;

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
    fn piece_get_color() {
        assert_eq!(expr(b"1+2"),         IResult::Done(&b""[..], 3));
        assert_eq!(expr(b"12+6-4+3"),    IResult::Done(&b""[..], 17));
        assert_eq!(expr(b"1+2*3+4"),     IResult::Done(&b""[..], 11));

        assert_eq!(expr(b"(2)"),         IResult::Done(&b""[..], 2));
        assert_eq!(expr(b"2*(3+4)"),     IResult::Done(&b""[..], 14));
        assert_eq!(expr(b"2*2/(5-1)+3"), IResult::Done(&b""[..], 4));
    }
}
