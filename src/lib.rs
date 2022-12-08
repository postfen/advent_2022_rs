pub mod puzzles;
use crate::puzzles::*;

pub fn print_header() {
    println!(
        r#"
  ┌─────────────────────┐
  │ Advent of Code 2022 │
  │     By postfen      │
  └─────────────────────┘
"#
    )
}
pub fn display_solutions() {
    print_header();
    day1::calories::print_solution();
    day2::rps::print_solution();
    day3::bags::print_solution();
    day4::campcleanup::print_solution();
    day5::supplystacks::print_solution();
    day6::tuningtrouble::print_solution();
    day7::nospaceleftondevice::print_solution();
}
