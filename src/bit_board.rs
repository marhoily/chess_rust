use pieces;
use pieces::*;
use colored_squares::*;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Line(u64);

#[derive(Debug, PartialEq)]
pub struct BitBoard([Line; COUNT]);

impl Line {
    fn test(self, square: SquareExp) -> bool {
        self.0 & square.bits() != 0
    }
    //fn set(&mut self, square: SquareExp) {
    //    self.0 |= square.bits()
    //}
    //fn count(self) -> u32 {
    //    self.0.count_ones()
    //}
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard([Line(0); COUNT])
    }
    fn line(&self, piece: Piece) -> Line {
        self.0[piece.bits() as usize]
    }
    pub fn check_square(&self, square: SquareExp) -> Piece {
        for piece in AllPieces {
            if self.line(piece).test(square) {
                return piece;
            }
        }
        pieces::EMPTY
    }
    pub fn set_piece(&mut self, square: SquareExp, piece: Piece) {
        self.0[piece.bits() as usize].0 |= square.bits();
    }
    pub fn get_piece(&self, square: SquareExp) -> Piece {
        for probe in AllPieces {
            if self.line(probe).test(square) {
                return probe;
            }
        }
        pieces::EMPTY
    }
    pub fn squares<'a>(&'a self) -> SquareIter<'a> {
        SquareIter {
            board: &self,
            square_iter: SquareExpIter::new(),
        }
    }
    pub fn dump(&self) -> String {
        use std::str::FromStr;
        let mut result : Vec<char> = String::from_str(
           " ╔═══╤═══╤═══╤═══╤═══╤═══╤═══╤═══╗\r\n\
            8║   │   │   │   │   │   │   │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            7║   │   │   │   │   │   │   │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            6║   │   │   │   │   │   │   │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            5║   │   │   │   │   │   │   │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            4║   │   │   │   │   │   │   │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            3║   │   │   │   │   │   │   │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            2║   │   │   │   │   │   │   │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            1║   │   │   │   │   │   │   │   ║\r\n \
             ╚═══╧═══╧═══╧═══╧═══╧═══╧═══╧═══╝\r\n   \
               A   B   C   D   E   F   G   H  \r\n")
             .unwrap().chars().collect();

        for index in 0..63 {
            let square = Square64::new(index);
            let piece = self.get_piece(square.to_exp());
            let (file, rank) = square.humanize();
            let i = (rank.bits() as usize * 2 + 1) * 36 + file.bits() as usize * 4 + 3;
            if piece != EMPTY {
                result[i] = piece.as_char();
            }
        }
        String::from(result)
    }
}

pub struct SquareIter<'a> {
    board: &'a BitBoard,
    square_iter: SquareExpIter,
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        self.square_iter.next().map(|square| self.board.get_piece(square))
    }
}

#[cfg(test)]
mod test {
    use colored_squares::*;
    use super::*;
    use std::iter::*;
    use pieces::*;

    #[test]
    fn check_square() {
        let mut b = BitBoard::new();
        b.set_piece(SquareExp::new(0b0001), BLACK_ROOK);
        b.set_piece(SquareExp::new(0b0100), BLACK_ROOK);
        assert_eq!(b.check_square(SquareExp::new(0b0001)), BLACK_ROOK);
        assert_eq!(b.check_square(SquareExp::new(0b0001)), BLACK_ROOK);
    }

    #[test]
    fn bit_board_squares() {
        let mut b = BitBoard::new();
        b.set_piece(SquareExp::new(0b0001), BLACK_ROOK);

        let all = b.squares()
            .collect::<Vec<Piece>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], BLACK_ROOK);
        assert_eq!(all[63], EMPTY);
    }

    #[test]
    fn dump() {
        use fen;
        let sample = b"1r2k2r/p2n1p1p/np4p1/2p1B1b1/7P/1P1P4/P1PN3P/RNQ2RK1";
        assert_eq!(fen::board(sample).unwrap().1.dump(),
           " ╔═══╤═══╤═══╤═══╤═══╤═══╤═══╤═══╗\r\n\
            8║   │ r │   │   │ k │   │   │ r ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            7║ p │   │   │ n │   │ p │   │ p ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            6║ n │ p │   │   │   │   │ p │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            5║   │   │ p │   │ B │   │ b │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            4║   │   │   │   │   │   │   │ P ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            3║   │ P │   │ P │   │   │   │   ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            2║ P │   │ P │ N │   │   │   │ P ║\r\n \
             ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n\
            1║ R │ N │ Q │   │   │ R │ K │   ║\r\n \
             ╚═══╧═══╧═══╧═══╧═══╧═══╧═══╧═══╝\r\n   \
               A   B   C   D   E   F   G   H  \r\n");
    }
}
