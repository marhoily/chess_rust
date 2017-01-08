// #![feature(question_mark)]
#![allow(unused_imports)]
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
// trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
 )]

#![feature(test)]
#![feature(associated_consts)]

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
pub mod side;
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

use file::*;
use rank::*;
use color::*;
use side::*;
use square::*;
use kind::*;
use piece::*;
use mask::*;
use bit_board::*;
use moves::*;
// use castle::*;
use position::*;
use analysis::*;
