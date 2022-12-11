// https://adventofcode.com/2022/day/5
pub fn print_solution() {
    let s = include_str!("input");
    let (s1, s2) = solve(s);
    println!("--- Day 5: Supply Stacks ---\nA. {s1}\nB. {s2}\n");
}

fn build_stack(boxes: &str) -> Vec<Vec<char>> {
    let (boxes, amt_str) = boxes.split_once('1').unwrap();
    let amt = amt_str
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..amt {
        stacks.push(Vec::new());
    }
    for line in boxes.lines().rev() {
        for (j, c) in line.bytes().enumerate() {
            if c.is_ascii_alphabetic() {
                stacks[j / 4].push(c as char);
            }
        }
    }
    stacks
}
fn parse_moves(instructions: &str) -> Vec<Vec<usize>> {
    instructions
        .lines()
        .map(|l| {
            l.split(' ')
                .filter_map(|word| word.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn get_top_boxes(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .fold(String::new(), |mut answer_str, next_stack| {
            answer_str.push(next_stack.last().unwrap().to_owned());
            answer_str
        })
}

fn solve(s: &str) -> (String, String) {
    let (stack_map, movelist_raw) = s.split_once("\n\n").unwrap();
    let (mut stacks, movelist) = (build_stack(stack_map), parse_moves(movelist_raw));
    let mut holding_stack: Vec<char> = Vec::new();
    let mut stacks2 = stacks.clone();
    for m in movelist {
        for _i in 0..m[0] {
            holding_stack.push(stacks[m[1] - 1].pop().unwrap());
            let cargo = stacks2[m[1] - 1].pop().unwrap();
            stacks2[m[2] - 1].push(cargo);
        }
        for _i in 0..m[0] {
            stacks[m[2] - 1].push(holding_stack.pop().unwrap());
        }
    }
    (get_top_boxes(&stacks2), get_top_boxes(&stacks))
}

#[cfg(test)]
#[test]
fn test_day5_1() {
    let s = include_str!("test_input");
    let s1 = solve(s).0;
    assert_eq!(s1, "CMZ");
}
#[test]
fn test_day5_2() {
    let s: &str = include_str!("test_input");
    let s2 = solve(s).1;
    assert_eq!(s2, "MCD");
}
