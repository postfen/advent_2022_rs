use std::{fmt::Display, ops::RangeInclusive};

pub fn print_solution() {
    let s = include_str!("input");
    let s1 = question_1(&s);
    let s2 = question_2(&s);
    println!(
        "--- Day 4: Camp Cleanup ---\n\
        A.  {s1}\n\
        B.  {s2}\n"
    )
}

trait Contains<T> {
    fn contains_either(&self, first: T, second: T) -> bool;
}

impl<T: PartialOrd> Contains<T> for RangeInclusive<T> {
    fn contains_either(&self, first: T, second: T) -> bool {
        (*self.start() <= first && first <= *self.end())
            | (*self.start() <= second && second <= *self.end())
    }
}

fn to_i32_vec(line: &str) -> Vec<i32> {
    let num_array: Vec<i32> = line
        .chars()
        .fold(String::new(), |mut num_str, next_char| {
            if next_char.is_digit(10) {
                num_str.push(next_char);
                num_str
            } else {
                num_str.push(',');
                num_str
            }
        })
        .split(',')
        .map(|x| x.to_owned().parse::<i32>().unwrap())
        .collect();
    num_array
}

fn question_1(s: &str) -> i32 {
    let mut total = 0;
    for line in s.lines() {
        let r = to_i32_vec(line);
        if (r[0] <= r[2] && r[1] >= r[3]) | (r[2] <= r[0] && r[3] >= r[1]) {
            total += 1
        }
    }
    total
}

fn question_2(s: &str) -> i32 {
    let mut total = 0;
    for line in s.lines() {
        let r = to_i32_vec(line);
        if ((r[0]..=r[1]).contains_either(r[2], r[3])) | ((r[2]..=r[3]).contains_either(r[0], r[1]))
        {
            total += 1
        }
    }
    total
}

