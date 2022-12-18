use std::collections::HashSet;

pub fn print_solution() {
    let s = include_str!("input");
    let s1 = q1(s);
    let s2 = q2(s);
    println!(
        "--- Day 9: Rope Bridge ---\n\
         A. {s1}\n\
         B. {s2}\n"
    );
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

fn move_in_direction(knot: &mut Coord, dir_char: &str) {
    match dir_char {
        "U" => knot.y -= 1,
        "D" => knot.y += 1,
        "L" => knot.x -= 1,
        "R" => knot.x += 1,
        &_ => panic!("Can't move in that direction"),
    }
}

fn q1(s: &str) -> i32 {
    let mut seen: HashSet<_> = HashSet::new();
    let start_pos = Coord { x: 0, y: 0 };
    let mut head = start_pos;
    let mut tail = start_pos;
    seen.insert(tail);
    for line in s.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<isize>().unwrap();
        for _ in 0..amount {
            move_in_direction(&mut head, direction);
            if not_touching(&head, &tail) {
                tail.x += (head.x - tail.x).signum();
                tail.y += (head.y - tail.y).signum();
                seen.insert(tail);
            }
        }
    }
    seen.len().try_into().unwrap()
}
fn not_touching(head: &Coord, tail: &Coord) -> bool {
    head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1
}

fn q2(s: &str) -> i32 {
    let mut seen: HashSet<_> = HashSet::new();
    let start_pos = Coord { x: 0, y: 0 };
    let mut rope = vec![start_pos; 10];
    seen.insert(start_pos);
    for line in s.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<isize>().unwrap();
        for _ in 0..amount {
            move_in_direction(&mut rope[0], direction);
            for head in 0..rope.len() - 1 {
                let tail = head + 1;
                if not_touching(&rope[head], &rope[tail]) {
                    rope[tail].x += (rope[head].x - rope[tail].x).signum();
                    rope[tail].y += (rope[head].y - rope[tail].y).signum();
                    if tail == rope.len() - 1 {
                        seen.insert(rope[rope.len()- 1]);
                    }
                }
            }
        }
    }
    seen.len().try_into().unwrap()
}

#[cfg(test)]
#[test]
fn test_dayn_a() {
    let s = include_str!("test_input");
    assert_eq!(q1(s), 13);
}
#[test]
fn test_dayn_b() {
    let s = include_str!("test_input2");
    assert_eq!(q2(s), 36);
}
