use super::*;

impl Mask {
    pub fn shift_north(self) -> Mask {
        self >> 8
    }
    pub fn shift_south(self) -> Mask {
        self << 8
    }
    pub fn shift_east(self) -> Mask {
        (self << 1) & !masks::files::A
    }
    pub fn shift_north_east(self) -> Mask {
        (self >> 7) & !masks::files::A
    }
    pub fn shift_south_east(self) -> Mask {
        (self << 9) & !masks::files::A
    }
    pub fn shift_west(self) -> Mask {
        (self >> 1) & !masks::files::H
    }
    pub fn shift_north_west(self) -> Mask {
        (self >> 9) & !masks::files::H
    }
    pub fn shift_south_west(self) -> Mask {
        (self << 7) & !masks::files::H
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn shift_north() {
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_north().dump(),
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
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_south().dump(),
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
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_east().dump(),
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
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_south_east().dump(),
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
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_north_east().dump(),
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
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_west().dump(),
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
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_south_west().dump(),
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
        let mask = masks::files::E | masks::ranks::_5;
        assert_eq!(mask.shift_north_west().dump(),
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