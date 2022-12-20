use std::collections::VecDeque;


pub fn print_solution() {
    let s = include_str!("input");
    let monkeys = parse_monkeys(s);
    let lcd: isize = monkeys.iter().map(|m| m.test).product();
    let s1 = solve(monkeys.clone(), 20, |x| x / 3);
    let s2 = solve(monkeys, 10000, |x| x % lcd);
    println!("--- Day 11: Monkey in the Middle ---\nA. {s1}\nB. {s2}\n");
}

#[derive(Debug, Clone)]
#[allow(unused)]
struct Monkey {
    id: u32,
    items: VecDeque<isize>,
    operation: Vec<String>,
    test: isize,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize,
}
impl Monkey {
    fn from(monkey_info: &str) -> Monkey {
        let mut monkey_info = monkey_info.lines();
        let id = monkey_info.next().unwrap().chars().rev().nth(1).unwrap().to_digit(10).unwrap();
        let items = monkey_info.next().unwrap().split([' ', ',']).filter_map(|x| x.parse::<isize>().ok()).collect::<VecDeque<isize>>();
        let operation = monkey_info.next().unwrap().split(": ").into_iter().nth(1).unwrap().split_whitespace().map(|x| x.to_owned()).collect();
        let test = monkey_info .next().unwrap().split(": ").into_iter().nth(1).unwrap().split_whitespace().last().unwrap().parse::<isize>().unwrap_or(0);
        let true_monkey = get_last_number(monkey_info.next().unwrap());
        let false_monkey = get_last_number(monkey_info.next().unwrap());
        let inspection_count = 0;
        Monkey {
            id,
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
            inspection_count,
        }
    }
}

fn get_last_number(s: &str) -> usize {
    s.split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn parse_monkeys(s: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_string in s.split("\n\n") {
        let raw_monkey_strings = monkey_string.split("\n\n").collect::<Vec<&str>>();
        for monkey_info in raw_monkey_strings {
            monkeys.push(Monkey::from(monkey_info));
        }
    }
    monkeys
}

fn operate(item: isize, operation: &[String]) -> isize {
    let operator = &operation[3];
    let operand = &operation[4].parse::<isize>().unwrap_or(item);
    let result = match operator.as_str() {
        "*" => item * operand,
        "+" => item + operand,
        _ => item,
    };
    result
}

fn process_monkey<R>(monkey: &mut Monkey, reduction: &R) ->Vec<(isize, usize)>
where
    R: Fn(isize) -> isize,
{
    let mut items_to_deliver: Vec<(isize, usize)> = Vec::new();
    monkey.inspection_count += monkey.items.len();
    for item in &mut monkey.items {
        let mut new_item = operate(*item, &monkey.operation);
        new_item = reduction(new_item);
        if new_item % monkey.test == 0 {
            items_to_deliver.push((new_item, monkey.true_monkey));
        } else {
            items_to_deliver.push((new_item, monkey.false_monkey));
        }
    }
    monkey.items.clear();

    items_to_deliver
}

fn solve<R>(mut monkeys: Vec<Monkey>, rounds: isize, reduction: R) -> usize
where
    R: Fn(isize) -> isize,
{
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            if monkeys[i].items.is_empty(){continue}
            let items = process_monkey(&mut monkeys[i], &reduction);
            for (item, recipient) in items {
                monkeys[recipient].items.push_back(item);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    monkeys[0].inspection_count * monkeys[1].inspection_count
}

#[cfg(test)]
#[test]
fn test_dayn_a() {
    let s = include_str!("test_input");
    let monkeys = parse_monkeys(s);
    assert_eq!(solve(monkeys, 20, |m| m / 3), 10605);
}
#[test]
fn test_dayn_b() {
    let s = include_str!("test_input");
    let monkeys = parse_monkeys(s);
    let lcd: isize = monkeys.iter().map(|m| m.test).product();
    assert_eq!(solve(monkeys, 10000, |x| x % lcd), 2_713_310_158);
}
