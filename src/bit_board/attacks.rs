use super::*;

impl BitBoard {
    pub fn is_attacked_by<S: Side>(&self, m: Mask) -> bool {
        self.pawns::<S>().attack().mask() & m != EMPTY ||
            self.knights::<S>().mask().knight_attacks() & m != EMPTY ||
            self.bishops::<S>().mask().bishop_attacks(self.occupation()) & m != EMPTY ||
            self.rooks::<S>().mask().rook_attacks(self.occupation()) & m != EMPTY
    }
    pub fn is_check_to<S: Side>(&self) -> bool {
        self.is_attacked_by::<S::Opposite>(self.kings::<S>().mask())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use position::*;

    #[test]
    fn no_check() {
        no("8/8/8/8/4K3/3P4/8/8 w - - 0 1")
    }

    #[test]
    fn to_black() {
        yes("8/8/8/8/4k3/3P4/8/8 b - - 0 1")
    }

    #[test]
    fn by_knight() {
        yes("8/8/8/8/4k3/2N5/8/8 b - - 0 1")
    }

    #[test]
    fn by_bishop() {
        yes("8/1B6/8/8/4k3/8/8/8 b - - 0 1")
    }
    #[test]
    fn bishop_is_blocked_with_white() {
        no("8/1B6/8/3R4/4k3/8/8/8 b - - 0 1")
    }
    #[test]
    fn bishop_is_blocked_with_black() {
        no("8/1B6/8/3r4/4k3/8/8/8 b - - 0 1")
    }
    #[test]
    fn by_rook() {
        yes("8/8/8/8/1R2k3/8/8/8 b - - 0 1")
    }
    #[test]
    fn rook_is_blocked_with_white() {
        no("8/8/8/8/1R1Bk3/8/8/8 b - - 0 1")
    }
    #[test]
    fn rook_is_blocked_with_black() {
        no("8/8/8/8/1R1bk3/8/8/8 b - - 0 1")
    }

    pub fn yes(fen: &str) {
        assert_eq!(check(fen), true);
    }

    pub fn no(fen: &str) {
        assert_eq!(check(fen), false);
    }

    pub fn check(fen: &str) -> bool {
        let p = Position::parse(fen);
        if p.active == Color::White {
            p.board.is_check_to::<White>()
        } else {
            p.board.is_check_to::<Black>()
        }
    }
}