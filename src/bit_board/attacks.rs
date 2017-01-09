use super::*;

impl BitBoard {
    pub fn is_attacked_by<S: Side>(&self, m: Mask) -> bool {
        if self.pawns::<S>().attack().mask() & m != EMPTY {
            return true
        }
        false
    }
    pub fn is_check<S: Side>(&self) -> bool {
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

    pub fn yes(fen: &str) {
        assert_eq!(check(fen), true);
    }

    pub fn no(fen: &str) {
        assert_eq!(check(fen), false);
    }

    pub fn check(fen: &str) -> bool {
        let p = Position::parse(fen);
        if p.active == Color::White {
            p.board.is_check::<White>()
        } else {
            p.board.is_check::<Black>()
        }
    }
}