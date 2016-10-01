// #![feature(question_mark)]
#![deny(
//missing_debug_implementations, missing_copy_implementations,
trivial_casts, trivial_numeric_casts,
unused_import_braces, unused_qualifications
 )]

#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;

pub mod geometry;
pub mod kind;
pub mod piece;
pub mod mask;
pub mod bit_board;
pub mod moves;
pub mod fen;
pub mod position;
pub mod analysis;
