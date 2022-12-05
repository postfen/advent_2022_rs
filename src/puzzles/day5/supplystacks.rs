// https://adventofcode.com/2022/day/5
pub fn print_solution() {
    let s = include_str!("input");
    let (s1, s2) = solve(&s);
    println!("--- Day 5: Supply Stacks ---\nA. {s1}\nB. {s2}\n")
}

fn build_stack(boxes: &str) -> Vec<Vec<char>> {
    let (boxes, amt_str) = boxes.split_once("1").unwrap();
    let amt = amt_str.trim().split(" ").last().unwrap().parse::<i32>().unwrap();

    // Create stacks (How to initialize these in one line? )
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..amt {
        stacks.push(Vec::new())
    }
    for line in boxes.lines().rev() {
        for (j, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                stacks[j / 4].push(c)
            }
        }
    }
    stacks
}
fn parse_moves(instructions: &str) -> Vec<Vec<usize>> {
    instructions
        .lines().map(|l| {l.split(" ")
                .filter_map(|word| word.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        }).collect()
}

fn move_boxes(mut stacks: Vec<Vec<char>>, tasklist: &Vec<Vec<usize>>) -> String {
    for task in tasklist {
        let amt = task[0];
        let sender = task[1] - 1;
        let receiver = task[2] - 1;
        for _i in 0..amt {
            let cargo = stacks[sender].pop().unwrap();
            stacks[receiver].push(cargo);
        }
    }
    get_top_boxes(&stacks)
}

fn move_boxes_ordered(mut stacks: Vec<Vec<char>>, tasklist: &Vec<Vec<usize>>) -> String {
    let mut holding_stack: Vec<char> = Vec::new();
    for task in tasklist {
        let amt = task[0];
        let sender = task[1] - 1;
        let receiver = task[2] - 1;
        for _i in 0..amt {
            holding_stack.push(stacks[sender].pop().unwrap());
        }
        for _i in 0..amt {
            stacks[receiver].push(holding_stack.pop().unwrap())
        }
    }
    get_top_boxes(&stacks)
}

fn get_top_boxes(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .fold(String::new(), |mut answer_str, next_stack| {
            answer_str.push(next_stack.last().unwrap().to_owned());
            answer_str
        })
}

fn solve(s: &str) -> (String, String) {
    let (stack_map, movelist_raw) = s.split_once("\n\n").unwrap();
    let stacks = build_stack(stack_map);
    let movelist = parse_moves(movelist_raw);
    let s1 = move_boxes(stacks.clone(), &movelist);
    let s2 = move_boxes_ordered(stacks.clone(), &movelist);
    (s1, s2)
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
    let s = include_str!("test_input");
    let s2 = solve(s).1;
    assert_eq!(s2, "MCD");
}
