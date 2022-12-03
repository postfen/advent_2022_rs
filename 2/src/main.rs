// https://adventofcode.com/2022/day/2

use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
trait Value {
    fn value(&self) -> u8;
}

trait Beats {
    fn beats(&self) -> Self;
}

trait Ties {
    fn ties(&self) -> Self;
}

trait Loses {
    fn loses(&self) -> Self;
}

impl Value for RPS {
    fn value(&self) -> u8 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl Beats for RPS {
    /// Returns RPS type that is defeated by caller
    fn beats(&self) -> Self {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

impl Ties for RPS {
    /// Returns RPS type that ties with caller
    fn ties(&self) -> Self {
        match self {
            RPS::Rock => RPS::Rock,
            RPS::Paper => RPS::Paper,
            RPS::Scissors => RPS::Scissors,
        }
    }
}

impl Loses for RPS {
    /// Returns RPS type that defeats caller
    fn loses(&self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

fn main() {
    let s = read_file_to_string("input").unwrap();
    question_1(&s);
    question_2(&s);
}

fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

fn question_1(input: &str) {
    let mut total: i32 = 0;
    for line in input.lines() {
        let round = line.split(' ').collect::<Vec<&str>>();
        let (opp_hand, player_hand) = (parse_move(round[0]), parse_move(round[1]));
        total += calc_score(&player_hand, &opp_hand) as i32;
    }
    println!("W/o Strategy:  {}", total);
}

fn question_2(input: &str) {
    let mut total: i32 = 0;
    for line in input.lines() {
        let round = line.split(' ').collect::<Vec<&str>>();
        let opp_hand = parse_move(round[0]);
        let player_hand = get_strategic_move(&opp_hand, round[1]);
        total += calc_score(&player_hand, &opp_hand) as i32;
    }
    println!("With Strategy: {}", total);
}

fn calc_score(p1: &RPS, p2: &RPS) -> u8 {
    let mut total_score = p1.value();
    if *p2 == p1.beats() {
        total_score += 6;
    } else if p2 == p1 {
        total_score += 3;
    }
    total_score
}

fn parse_move(play: &str) -> RPS {
    match play {
        "A" | "X" => RPS::Rock,
        "B" | "Y" => RPS::Paper,
        "C" | "Z" => RPS::Scissors,
        _ => panic!("Illegal play"),
    }
}

fn get_strategic_move(opp: &RPS, play: &str) -> RPS {
    match play {
        "X" => opp.beats(),
        "Y" => opp.ties(),
        "Z" => opp.loses(),
        _ => panic!("Illegal play"),
    }
}
