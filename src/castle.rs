#![allow(dead_code)]
#![allow(trivial_casts, trivial_numeric_casts)]

// todo: make castle be masks for squares that need checking?
// todo: let's keep parser near the struct
bitflags! {
    pub flags Castle: u8 {
        const NONE = 0,
        const Q = WQ.bits | BQ.bits,
        const K = WK.bits | BK.bits,
        const W = WQ.bits | WK.bits,
        const B = BQ.bits | BK.bits,
        const WQ = 1 << 0,
        const WK = 1 << 2,
        const BQ = 1 << 3,
        const BK = 1 << 4,
        const ALL = Q.bits | K.bits,
    }
}
