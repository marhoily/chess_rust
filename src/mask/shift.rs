use super::*;

impl Mask {
    pub fn shift_north(self) -> Mask {
        self >> 8
    }
    pub fn shift_south(self) -> Mask {
        self << 8
    }
    pub fn shift_east(self) -> Mask {
        (self << 1) & !masks::A
    }
    pub fn shift_north_east(self) -> Mask {
        (self >> 7) & !masks::A
    }
    pub fn shift_south_east(self) -> Mask {
        (self << 9) & !masks::A
    }
    pub fn shift_west(self) -> Mask {
        (self >> 1) & !masks::H
    }
    pub fn shift_north_west(self) -> Mask {
        (self >> 9) & !masks::H
    }
    pub fn shift_south_west(self) -> Mask {
        (self << 7) & !masks::H
    }
}

#[cfg(test)]
mod tests {
    use super::super::masks::E;
    use super::super::masks::_5;

    //  E|5:
    //           |^^^^@^^^|
    //           |^^^^@^^^|
    //           |^^^^@^^^|
    //           |@@@@@@@@|
    //           |^^^^@^^^|
    //           |^^^^@^^^|
    //           |^^^^@^^^|
    //           |^^^^@^^^|

    #[test]
    fn shift_north() {
        assert_eq!((E | _5).shift_north().dump(),
                   "|^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |@@@@@@@@|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^^^^^|...");
    }

    #[test]
    fn shift_south() {
        assert_eq!((E | _5).shift_south().dump(),
                   "|^^^^^^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |@@@@@@@@|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...\
                    |^^^^@^^^|...");
    }

    #[test]
    fn shift_east() {
        assert_eq!((E | _5).shift_east().dump(),
                   "|^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^@@@@@@@|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...");
    }

    #[test]
    fn shift_south_east() {
        assert_eq!((E | _5).shift_south_east().dump(),
                   "|^^^^^^^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^@@@@@@@|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...");
    }

    #[test]
    fn shift_north_east() {
        assert_eq!((E | _5).shift_north_east().dump(),
                   "|^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^@@@@@@@|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^@^^|...\
                    |^^^^^^^^|...");
    }

    #[test]
    fn shift_west() {
        assert_eq!((E | _5).shift_west().dump(),
                   "|^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |@@@@@@@^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...");
    }

    #[test]
    fn shift_south_west() {
        assert_eq!((E | _5).shift_south_west().dump(),
                   "|^^^^^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |@@@@@@@^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...");
    }

    #[test]
    fn shift_north_west() {
        assert_eq!((E | _5).shift_north_west().dump(),
                   "|^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |@@@@@@@^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^@^^^^|...\
                    |^^^^^^^^|...");
    }
}
