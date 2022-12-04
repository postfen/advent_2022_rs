// https://adventofcode.com/2022/day/3
use std::collections::HashSet;
use std::ops::BitAnd;

trait Value {
    fn value(&self) -> u8;
}

impl Value for char {
    fn value(&self) -> u8 {
        if self.is_lowercase() {
            return *self as u8 - b'`'; // a -> 1
        } else {
            return *self as u8 - b'&'; // A -> 27
        }
    }
}

fn question_1(s: &str) -> i32 {
    let mut total: i32 = 0;
    for line in s.lines() {
        let (a, b) = &line.split_at(line.len() / 2);
        let a_set: HashSet<char> = a.chars().collect();
        let b_set: HashSet<char> = b.chars().collect();
        total += a_set.bitand(&b_set).into_iter().next().unwrap().value() as i32;
    }
    total
}

fn question_2(s: &str) -> i32 {
    let mut total: i32 = 0;
    let mut lines = s.lines();
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let a: HashSet<char> = a.chars().collect();
        let b: HashSet<char> = b.chars().collect();
        let c: HashSet<char> = c.chars().collect();
        total += c.bitand(&a.bitand(&b)).into_iter().next().unwrap().value() as i32;
    }
    total
}

pub fn print_solution() {
    let s = include_str!("input");
    let s1 = question_1(&s);
    let s2 = question_2(&s);
    println!(
        "--- Day 3: Rucksack Reorganization ---\n\
        A. {s1}\n\
        B. {s2}\n"
    )
}
