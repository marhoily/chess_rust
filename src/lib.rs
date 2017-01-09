// #![feature(question_mark)]
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    // trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    unused_imports
)]

#![feature(test, associated_consts, plugin)]
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
pub mod side;
pub mod sided_mask;
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

mod check_namespaces;
