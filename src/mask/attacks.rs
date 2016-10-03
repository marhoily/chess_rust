use super::*;
use super::masks::files::*;

impl Mask {
    pub fn white_pawn_attacks(self) -> Mask {
        self.shift_north_east() | self.shift_north_west()
    }
    pub fn black_pawn_attacks(self) -> Mask {
        self.shift_south_east() | self.shift_south_west()
    }
    pub fn knight_attacks(self) -> Mask {
        let x = self;
        let a = ((x << 17) | (x >> 15)) & !A;
        let b = ((x << 10) | (x >> 6)) & !(A | B);
        let c = ((x << 15) | (x >> 17)) & !H;
        let d = ((x << 6) | (x >> 10)) & !(G | H);
        a | b | c | d
    }
    pub fn bishop_attacks(self, stoppers: Mask) -> Mask {
        self.fill(Mask::shift_north_east, stoppers).shift_north_east() |
        self.fill(Mask::shift_north_west, stoppers).shift_north_west() |
        self.fill(Mask::shift_south_east, stoppers).shift_south_east() |
        self.fill(Mask::shift_south_west, stoppers).shift_south_west()
    }
    pub fn rook_attacks(self, stoppers: Mask) -> Mask {
        self.fill(Mask::shift_north, stoppers).shift_north() |
        self.fill(Mask::shift_south, stoppers).shift_south() |
        self.fill(Mask::shift_east, stoppers).shift_east() |
        self.fill(Mask::shift_west, stoppers).shift_west()
    }
    pub fn queen_attacks(self, stoppers: Mask) -> Mask {
        self.rook_attacks(stoppers) | self.bishop_attacks(stoppers)
    }
    pub fn king_attacks(self) -> Mask {
        let dots = self.shift_east() | self.shift_west();
        let line = self | dots;
        dots | line.shift_north() | line.shift_south()
    }
}

#[cfg(test)]
mod tests {
    use super::super::masks::files::*;
    use super::super::masks::ranks::*;
    use super::super::masks::*;

    #[test]
    fn white_pawn_attacks() {
        assert_eq!((A7 | E7 | F8 | H7 | B3 | G3 | A1 | H1)
                       .white_pawn_attacks()
                       .dump(),
                   "|^@^@^@@^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |@^@^^@^@|...\
                    |^^^^^^^^|...\
                    |^@^^^^@^|...\
                    |^^^^^^^^|...");
    }
    #[test]
    fn black_pawn_attacks() {
        assert_eq!((A2 | E2 | F1 | H2 | B6 | G6 | A8 | H8)
                       .black_pawn_attacks()
                       .dump(),
                   "|^^^^^^^^|...\
                    |^@^^^^@^|...\
                    |^^^^^^^^|...\
                    |@^@^^@^@|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |^@^@^@@^|...");
    }

    #[test]
    fn knight_attacks() {
        assert_eq!(E5.knight_attacks().dump(),
                   "|^^^^^^^^|...\
                    |^^^@^@^^|...\
                    |^^@^^^@^|...\
                    |^^^^^^^^|...\
                    |^^@^^^@^|...\
                    |^^^@^@^^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...");
    }
    #[test]
    fn knight_attacks_corners() {
        assert_eq!((A8 | A1 | H1 | H8).knight_attacks().dump(),
                   "|^^^^^^^^|...\
                    |^^@^^@^^|...\
                    |^@^^^^@^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |^@^^^^@^|...\
                    |^^@^^@^^|...\
                    |^^^^^^^^|...");
    }
    #[test]
    fn knight_attacks_flanks() {
        assert_eq!((A3 | A6 | H3 | H6).knight_attacks().dump(),
                   "|^@^^^^@^|...\
                    |^^@^^@^^|...\
                    |^^^^^^^^|...\
                    |^@@^^@@^|...\
                    |^@@^^@@^|...\
                    |^^^^^^^^|...\
                    |^^@^^@^^|...\
                    |^@^^^^@^|...");
    }
    #[test]
    fn knight_attacks_inner() {
        assert_eq!((B3 | G6).knight_attacks().dump(),
                   "|^^^^^@^@|...\
                    |^^^^@^^^|...\
                    |^^^^^^^^|...\
                    |@^@^@^^^|...\
                    |^^^@^@^@|...\
                    |^^^^^^^^|...\
                    |^^^@^^^^|...\
                    |@^@^^^^^|...");
    }

    #[test]
    fn bishop_attacks() {
        assert_eq!(F7.bishop_attacks(B | _2).dump(),
                   "|^^^^@^@^|...\
                    |^^^^^^^^|...\
                    |^^^^@^@^|...\
                    |^^^@^^^@|...\
                    |^^@^^^^^|...\
                    |^@^^^^^^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...");
    }
    #[test]
    fn rook_attacks() {
        assert_eq!(F7.rook_attacks(B | _2).dump(),
                   "|^^^^^@^^|...\
                    |^@@@@^@@|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^^^^|...");
    }
    #[test]
    fn queen_attacks() {
        assert_eq!(F7.queen_attacks(B | _2).dump(),
                   "|^^^^@@@^|...\
                    |^@@@@^@@|...\
                    |^^^^@@@^|...\
                    |^^^@^@^@|...\
                    |^^@^^@^^|...\
                    |^@^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^^^^|...");
    }
    #[test]
    fn king_attacks() {
        assert_eq!((F7|C1).king_attacks().dump(),
                   "|^^^^@@@^|...\
                    |^^^^@^@^|...\
                    |^^^^@@@^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |^^^^^^^^|...\
                    |^@@@^^^^|...\
                    |^@^@^^^^|...");
    }
}

// pub fn white_pawn_pushes
// pub fn black_pawn_pushes
// pub fn xray_bishop_attacks(self, occupied: Mask, stoppers: Mask) -> Mask {
//     let attacks = self.bishop_attacks(occupied);
//     attacks ^ self.bishop_attacks((stoppers & attacks) ^ stoppers)
