use piece::{Piece, pieces};
use piece::pieces::All;
use mask::{masks, Mask};
use mask::masks::MaskIter;
use geometry::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct BitBoard([Mask; pieces::COUNT]);

impl BitBoard {
    pub fn new() -> Self {
        BitBoard([masks::EMPTY; pieces::COUNT])
    }
    fn line(&self, piece: Piece) -> Mask {
        self.0[piece.bits() as usize]
    }
    pub fn check_square(&self, square: Mask) -> Piece {
        for piece in All {
            if self.line(piece).test(square) {
                return piece;
            }
        }
        pieces::VOID
    }
    pub fn set_piece(&mut self, square: Mask, piece: Piece) {
        let idx = piece.bits() as usize;
        self.0[idx] = self.0[idx].with(square);
    }
    pub fn get_piece(&self, square: Mask) -> Piece {
        for probe in All {
            if self.line(probe).test(square) {
                return probe;
            }
        }
        pieces::VOID
    }
    pub fn squares(&self) -> SquareIter {
        SquareIter {
            board: self,
            current: MaskIter::new(),
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
            let square = Square::new(index);
            let piece = self.get_piece(square.mask());
            let (file, rank) = square.file_rank();
            let i = (rank.bits() as usize * 2 + 1) * 36 + file.bits() as usize * 4 + 3;
            if piece != pieces::VOID {
                result[i] = piece.char();
            }
        }
        String::from(result)
    }
    pub fn parse(input: &str) -> Self {
        use fen;

        fen::parse_bit_borad(input.as_bytes()).unwrap().1
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SquareIter<'a> {
    board: &'a BitBoard,
    current: MaskIter,
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.next().map(|square| self.board.get_piece(square))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::*;
    use piece::*;
    use mask::Mask;

    #[test]
    fn check_square() {
        let mut b = BitBoard::new();
        b.set_piece(Mask::new(0b0001), pieces::BLACK_ROOK);
        b.set_piece(Mask::new(0b0100), pieces::BLACK_ROOK);
        assert_eq!(b.check_square(Mask::new(0b0001)), pieces::BLACK_ROOK);
        assert_eq!(b.check_square(Mask::new(0b0001)), pieces::BLACK_ROOK);
    }

    #[test]
    fn bit_board_squares() {
        let mut b = BitBoard::new();
        b.set_piece(Mask::new(0b0001), pieces::BLACK_ROOK);

        let all = b.squares().collect::<Vec<_>>();
        assert_eq!(all.len(), 64);
        assert_eq!(all[0], pieces::BLACK_ROOK);
        assert_eq!(all[63], pieces::VOID);
    }

    #[test]
    fn dump() {
        let sample = "1r2k2r/p2n1p1p/np4p1/2p1B1b1/7P/1P1P4/P1PN3P/RNQ2RK1";
        assert_eq!(BitBoard::parse(sample).dump(),
                   " ╔═══╤═══╤═══╤═══╤═══╤═══╤═══╤═══╗\r\n8║   │ r │   │   │ k │   │   │ r ║\r\n \
                    ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n7║ p │   │   │ n │   │ p │   │ p ║\r\n \
                    ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n6║ n │ p │   │   │   │   │ p │   ║\r\n \
                    ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n5║   │   │ p │   │ B │   │ b │   ║\r\n \
                    ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n4║   │   │   │   │   │   │   │ P ║\r\n \
                    ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n3║   │ P │   │ P │   │   │   │   ║\r\n \
                    ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n2║ P │   │ P │ N │   │   │   │ P ║\r\n \
                    ╟───┼───┼───┼───┼───┼───┼───┼───╢\r\n1║ R │ N │ Q │   │   │ R │ K │   ║\r\n \
                    ╚═══╧═══╧═══╧═══╧═══╧═══╧═══╧═══╝\r\n   A   B   C   D   E   F   G   H  \r\n");
    }
}
