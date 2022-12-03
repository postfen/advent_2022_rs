use std::fs::File;
use std::io;
use std::io::prelude::*;

// https://adventofcode.com/2022/day/1

fn main() {
    let s = read_file_to_string("input").unwrap();
    question_1(&s);
    question_2(&s);
}

fn question_1(s: &str) {
    let mut count = 0;
    let mut max = 0;
    for line in s.lines() {
        if line.len() == 0 {
            if count > max {
                max = count;
            }
            count = 0;
        } else {
            let n: i32 = line.trim().parse().unwrap_or(0);
            count += n;
        }
    }
    println!("Highest Calories: {max}")
}

fn question_2(s: &str) {
    let mut elves: Vec<i32> = vec![];
    let mut total = 0;
    for line in s.lines() {
        if line.len() == 0 {
            elves.push(total);
            total = 0;
        } else {
            total += line.trim().parse::<i32>().unwrap_or(0);
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    println!("Sum of 3 Highest: {}", elves[0] + elves[1] + elves[2])
}

fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}
