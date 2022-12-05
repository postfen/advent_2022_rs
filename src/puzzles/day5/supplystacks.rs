// https://adventofcode.com/2022/day/5
pub fn print_solution() {
    let s = include_str!("input");
    let s1 = question_1(&s);
    let s2 = question_2(&s);
    println!(
        "--- Day 5: Supply Stacks ---\n\
         A. {s1}\n\
         B. {s2}\n"
    )
}

fn build_stack(boxes: &str) -> Vec<Vec<char>> {
    // get amount of stacks to create
    let amt = &boxes.lines().next_back().unwrap().trim().split(" ").last()
                    .unwrap().parse::<i32>().unwrap_or_default();

    // Create stacks (How to initialize these in one line? )
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..*amt {
        stacks.push(Vec::new())
    }
    // remove number line from input string
    let boxes = boxes.splitn(2, "\n 1").next().unwrap();

    // add cargo to stacks
    for line in boxes.lines().rev() {
        for (j, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                stacks[j / 4].push(c)
            }
        }
    }
    stacks
}
fn parse_instructions(instructions: &str) -> Vec<Vec<usize>> {
    instructions
        .lines()
        .map(|l| {l.split(" ").filter_map(|word| word.parse::<usize>().ok())
                              .collect::<Vec<usize>>()}).collect()
}

fn move_boxes(stacks: &mut Vec<Vec<char>>, instructions: &str) {
    // Get Instructions
    let tasklist = parse_instructions(instructions);

    // For each vec in movelist,
    for task in tasklist {
        let amt = task[0];
        let sender = task[1] - 1;
        let receiver = task[2] - 1;
        for _i in 0..amt {
            let cargo = stacks[sender].pop().unwrap();
            stacks[receiver].push(cargo);
        }
    }
}

fn move_boxes_ordered(stacks: &mut Vec<Vec<char>>, instructions: &str) {
    // Get Instructions
    let tasklist = parse_instructions(instructions);
    let mut holding_stack: Vec<char> = Vec::new();
    // For each vec in movelist,
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
}

fn get_top_boxes(stacks: Vec<Vec<char>>) -> String{
    stacks
        .iter()
        .fold(String::new(), |mut answer_str, next_stack| {
            answer_str.push(next_stack.last().unwrap().to_owned());
            answer_str
        })
}

fn question_1(s: &str) -> String {
    let (stack_map, movelist) = s.split_once("\n\n").unwrap();
    let mut stacks = build_stack(stack_map);
    move_boxes(&mut stacks, movelist);
    get_top_boxes(stacks)
}

fn question_2(s: &str) -> String {
    let (stack_map, movelist) = s.split_once("\n\n").unwrap();
    let mut stacks = build_stack(stack_map);
    move_boxes_ordered(&mut stacks, movelist);
    get_top_boxes(stacks)
}

#[cfg(test)]
#[test]
fn test_day5_1() {
    let s = include_str!("test_input");
    assert_eq!(question_1(&s), "CMZ");
}
#[test]
fn test_day5_2() {
    let s = include_str!("test_input");
    assert_eq!(question_2(&s), "MCD");
}
