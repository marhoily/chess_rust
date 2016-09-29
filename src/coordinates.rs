use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Display;
use std::fmt::Formatter;
use nom::IResult;
use nom::IResult::*;
use std;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct File(i8);

impl File {
    pub fn new(num: i8) -> Self {
        File(num)
    }
    pub fn parse(letter: char) -> Self {
        File(letter as i8 - 'a' as i8)
    }
    pub fn char(self) -> char {
        ('a' as u8 + self.0 as u8) as char
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Rank(i8);

impl Rank {
    pub fn new(num: i8) -> Self {
        Rank(num)
    }
    pub fn parse(number: char) -> Self {
        Rank('8' as i8 - number as i8)
    }
    pub fn char(self) -> char {
        ('8' as u8 - self.0 as u8) as char
    }
}

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.char())
    }
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn invert(self) -> Self {
        if self == Color::Black {
            Color::White
        } else {
            Color::Black
        }
    }
}

// Note that index 0 corresponds to a8, and NOT a1!
// Indexes read left to right, top to bottom!
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Square64(i8);

type Spr = std::result::Result<Square64,ParseCoordinateError>;
impl Square64 {
    pub fn new(square_number: i8) -> Self {
        Square64(square_number)
    }
    pub fn from(f: File, r: Rank) -> Self {
        Square64(f.0 + r.0 * 8)
    }
    pub fn parse(coordinate: &str) -> Square64 {
        Square64::try_parse(coordinate).unwrap()
    }
    pub fn try_parse(coordinate: &str) -> Spr {
        use nom::{Err, ErrorKind};
        match Square64::parse_nom(coordinate.as_bytes()) {
            Done(_, square) => Ok(square),
            Error(Err::Position(ErrorKind::Custom(code), _)) => Err(code),
            Incomplete(_) => Err(ParseCoordinateError::Incomplete),
            _ => panic!("custom error!?")
        }
    }
    pub fn parse_nom(input: &[u8]) -> IResult<&[u8], Square64, ParseCoordinateError> {
        use nom::{Err, ErrorKind, Needed};
        let mut consumed = 0;
        let mut file: Option<File> = None;
        let mut rank: Option<Rank> = None;
        for &e in input {
            match consume(e as char) {
                Token::File(f) => {
                    if file.is_some() {
                        panic!("1")
                        //return Error(Err::Position(ErrorKind::Custom(ParseCoordinateError::UnrecognizedToken),
                        //                           &input[consumed..]));
                    }

                    file = Some(f)
                }
                Token::Rank(r) => {
                    if file.is_none() {
                        return Error(Err::Position(ErrorKind::Custom(ParseCoordinateError::RankFirst),
                                                   &input[consumed..]));
                    }

                    rank = Some(r)
                }
                Token::Other => {
                    panic!("3")
                    //return Error(Err::Position(ErrorKind::Custom(ParseCoordinateError::UnrecognizedToken),
                    //                           &input[consumed..]))
                }
            }
            consumed += 1;
        }
        if consumed == 2 {
            Done(&input[2..], Square64::from(file.unwrap(), rank.unwrap()))
        } else {
            Incomplete(Needed::Size(2))
        }
    }
    pub fn to_exp(&self) -> SquareExp {
        SquareExp(1 << self.0)
    }
    pub fn bits(self) -> i8 {
        self.0
    }
    pub fn humanize(self) -> (File, Rank) {
        (File(self.0 % 8), Rank(self.0 / 8))
    }
    pub fn color(self) -> Color {
        let (file, rank) = self.humanize();
        if (file.0 % 2) == (rank.0 % 2) {
            Color::White
        } else {
            Color::Black
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseCoordinateError {
    UnrecognizedToken,
    Incomplete,
    RankFirst
}
enum Token {
    File(File),
    Rank(Rank),
    Other,
}

fn consume(c: char) -> Token {
    match c {
        'a'...'h' => Token::File(File::parse(c)),
        '1'...'8' => Token::Rank(Rank::parse(c)),
        _ => Token::Other,
    }
}


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct SquareExp(u64);

impl SquareExp {
    pub fn new(exp: u64) -> Self {
        SquareExp(exp)
    }
    pub fn bits(self) -> u64 {
        self.0
    }
    pub fn is_out(&self) -> bool {
        self.0 == 0
    }
    pub fn next(&mut self) {
        self.0 <<= 1;
    }
    pub fn forward(&mut self, count: u8) {
        self.0 <<= count;
    }
}

pub struct AllSquaresExp;

impl IntoIterator for AllSquaresExp {
    type Item = SquareExp;
    type IntoIter = SquareExpIter;

    fn into_iter(self) -> Self::IntoIter {
        SquareExpIter::new()
    }
}

pub struct SquareExpIter(u64);

impl SquareExpIter {
    pub fn new() -> Self {
        SquareExpIter(1)
    }
}

impl Iterator for SquareExpIter {
    type Item = SquareExp;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let result = SquareExp(self.0);
            self.0 <<= 1;
            Some(result)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::iter::*;

    #[test]
    fn color_invert() {
        assert_eq!(Color::White.invert(), Color::Black);
        assert_eq!(Color::Black.invert(), Color::White);
    }

    #[test]
    fn file_char() {
        assert_eq!(File::new(0).char(), 'a');
        assert_eq!(File::new(1).char(), 'b');
        assert_eq!(File::new(2).char(), 'c');
        assert_eq!(File::new(3).char(), 'd');
        assert_eq!(File::new(4).char(), 'e');
        assert_eq!(File::new(5).char(), 'f');
        assert_eq!(File::new(6).char(), 'g');
        assert_eq!(File::new(7).char(), 'h');
    }

    #[test]
    fn rank_char() {
        assert_eq!(Rank::new(0).char(), '8');
        assert_eq!(Rank::new(1).char(), '7');
        assert_eq!(Rank::new(2).char(), '6');
        assert_eq!(Rank::new(3).char(), '5');
        assert_eq!(Rank::new(4).char(), '4');
        assert_eq!(Rank::new(5).char(), '3');
        assert_eq!(Rank::new(6).char(), '2');
        assert_eq!(Rank::new(7).char(), '1');
    }

    #[test]
    fn file_parse() {
        assert_eq!(File::parse('a').0, 0);
        assert_eq!(File::parse('b').0, 1);
        assert_eq!(File::parse('c').0, 2);
        assert_eq!(File::parse('d').0, 3);
        assert_eq!(File::parse('e').0, 4);
        assert_eq!(File::parse('f').0, 5);
        assert_eq!(File::parse('g').0, 6);
        assert_eq!(File::parse('h').0, 7);
    }

    #[test]
    fn rank_parse() {
        assert_eq!(Rank::parse('8').0, 0);
        assert_eq!(Rank::parse('7').0, 1);
        assert_eq!(Rank::parse('6').0, 2);
        assert_eq!(Rank::parse('5').0, 3);
        assert_eq!(Rank::parse('4').0, 4);
        assert_eq!(Rank::parse('3').0, 5);
        assert_eq!(Rank::parse('2').0, 6);
        assert_eq!(Rank::parse('1').0, 7);
    }

    #[test]
    fn square_parse() {
        assert_eq!(Square64::parse("a8").0, 0);
        assert_eq!(Square64::parse("b7").0, 9);
        assert_eq!(Square64::parse("c6").0, 18);
        assert_eq!(Square64::parse("d5").0, 27);
        assert_eq!(Square64::parse("e4").0, 36);
        assert_eq!(Square64::parse("f3").0, 45);
        assert_eq!(Square64::parse("g2").0, 54);
        assert_eq!(Square64::parse("h1").0, 63);
    }
    #[test]
    fn rank_first() {
        assert_eq!(Square64::try_parse("8").unwrap_err(), 
            ParseCoordinateError::RankFirst);
    }
    #[test]
    fn rank_first() {
        assert_eq!(Square64::try_parse("8").unwrap_err(),
            ParseCoordinateError::RankFirst);
    }

    #[test]
    fn all_squares_exp() {
        let all = AllSquaresExp.into_iter()
            .collect::<Vec<SquareExp>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], SquareExp(1));
        assert_eq!(all[63], SquareExp(1 << 63));
    }
}
