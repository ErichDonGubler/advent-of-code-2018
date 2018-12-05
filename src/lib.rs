#![deny(warnings)]

extern crate aoc_runner;
extern crate aoc_runner_derive;
extern crate arrayvec;
extern crate re_parse;
extern crate serde;
extern crate serde_derive;

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;

aoc_lib!{ year = 2018 }
