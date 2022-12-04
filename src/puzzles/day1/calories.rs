// https://adventofcode.com/2022/day/1
pub fn print_solution() {
    let s = include_str!("input");
    let s1 = question_1(&s);
    let s2 = question_2(&s);
    println!(
        "--- Day 1: Calorie Counting ---\n\
         A. {s1}\n\
         B. {s2}\n"
    )
}

pub fn question_1(s: &str) -> i32 {
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
    max
}

pub fn question_2(s: &str) -> i32 {
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
    elves[0] + elves[1] + elves[2]
}
