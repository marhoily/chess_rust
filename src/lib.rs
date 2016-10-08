// #![feature(question_mark)]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
// trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
 )]

#![feature(test)]

#![feature(plugin)]
#![plugin(clippy)]

extern crate test;
extern crate rand;
extern crate itertools;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;

pub mod file;
pub mod rank;
pub mod color;
pub mod square;
#[cfg(test)]
mod square88;
pub mod kind;
pub mod piece;
pub mod mask;
pub mod bit_board;
#[cfg(test)]
mod board88;
pub mod moves;
pub mod castle;
pub mod position;
pub mod analysis;
