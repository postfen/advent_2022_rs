// https://adventofcode.com/2022/day/1
pub fn print_solution() {
    let s = include_str!("input");
    let s1 = question_1(s);
    let s2 = question_2(s);
    println!(
        "--- Day 1: Calorie Counting ---\n\
         A. {s1}\n\
         B. {s2}\n"
    );
}


fn question_1(s: &str) -> i32 {
    let sum = s
        .split("\n\n")
        .map(|elf_load| {
            {
                elf_load
                    .lines()
                    .map(|x| x.parse::<i32>().unwrap_or_default())
            }
            .sum::<i32>()
        })
        .max()
        .unwrap_or_default();
    sum
}

fn question_2(s: &str) -> u32 {
    let mut elves = s
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum()
}

#[allow(dead_code)]
fn question_1_v1(s: &str) -> i32 {
    let mut count = 0;
    let mut max = 0;
    for line in s.lines() {
        if line.is_empty() {
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

#[allow(dead_code)]
fn question_2_v1(s: &str) -> i32 {
    let mut elves: Vec<i32> = vec![];
    let mut total = 0;
    for line in s.lines() {
        if line.is_empty() {
            elves.push(total);
            total = 0;
        } else {
            total += line.trim().parse::<i32>().unwrap_or(0);
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    println!("{} {} {}", elves[0], elves[1], elves[2]);
    elves[0] + elves[1] + elves[2]
}

#[cfg(test)]
#[test]
fn test_day1_a() {
    let s = include_str!("test_input");
    assert_eq!(question_1(s), 24000);
}
#[test]
fn test_day1_b() {
    let s = include_str!("test_input");
    assert_eq!(question_2(s), 45000);
}
