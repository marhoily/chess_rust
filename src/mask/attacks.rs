use super::*;
use super::masks::files::*;

impl Mask {
    pub fn knight_attacks(self) -> Mask {
        let x = self;
        let a = ((x << 17) | (x >> 15)) & !A;
        let b = ((x << 10) | (x >> 6)) & !(A | B);
        let c = ((x << 15) | (x >> 17)) & !H;
        let d = ((x << 6) | (x >> 10)) & !(G | H);
        a | b | c | d
    }
}

#[cfg(test)]
mod tests {
    use super::super::masks::*;

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
        assert_eq!((A8|A1|H1|H8).knight_attacks().dump(),
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
        assert_eq!((A3|A6|H3|H6).knight_attacks().dump(),
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
        assert_eq!((B3|G6).knight_attacks().dump(),
                   "|^^^^^@^@|...\
                    |^^^^@^^^|...\
                    |^^^^^^^^|...\
                    |@^@^@^^^|...\
                    |^^^@^@^@|...\
                    |^^^^^^^^|...\
                    |^^^@^^^^|...\
                    |@^@^^^^^|...");
    }
}
