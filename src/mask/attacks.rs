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
    use super::super::masks::E5;

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
}
