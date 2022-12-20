use std::collections::HashSet;
/*
                             --- Day 9: Rope Bridge ---
                              A. 6406
                              B. 2643
*/

pub fn print_solution() {
    let s = include_str!("input");
    let (s1, s2) = solve(s);
    println!("--- Day 9: Rope Bridge ---\nA. {s1}\nB. {s2}\n");
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn not_touching(&self, other: &Coord) -> bool {
        self.x.abs_diff(other.x) > 1 || self.y.abs_diff(other.y) > 1
    }

    fn move_by_coords(&mut self, dir_char: &str) {
        match dir_char {
            "U" => self.y -= 1,
            "D" => self.y += 1,
            "L" => self.x -= 1,
            "R" => self.x += 1,
            _ => panic!("Can't move in that direction"),
        }
    }
}

fn solve(input: &str) -> (i32, i32) {
    let mut s1: HashSet<_> = HashSet::new();
    let mut s2: HashSet<_> = HashSet::new();
    let start_pos = Coord { x: 0, y: 0 };
    let mut rope = vec![start_pos; 10];
    let mut head = start_pos;
    let mut tail = start_pos;
    s1.insert(tail);
    s2.insert(start_pos);
    for line in input.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<isize>().unwrap();
        for _ in 0..amount {
            head.move_by_coords(direction);
            rope[0].move_by_coords(direction);
            if head.not_touching(&tail) {
                tail.x += (head.x - tail.x).signum();
                tail.y += (head.y - tail.y).signum();
                s1.insert(tail);
            }
            for first in 0..rope.len() - 1 {
                let second = first + 1;
                if rope[first].not_touching(&rope[second]) {
                    rope[second].x += (rope[first].x - rope[second].x).signum();
                    rope[second].y += (rope[first].y - rope[second].y).signum();
                    if second == rope.len() - 1 {
                        s2.insert(rope[rope.len() - 1]);
                    }
                }
            }
        }
    }
    (s1.len().try_into().unwrap(), s2.len().try_into().unwrap())
}

#[cfg(test)]
#[test]
fn test_dayn_a() {
    let s = include_str!("test_input");
    assert_eq!(solve(s).0, 13);
}
#[test]
fn test_dayn_b() {
    let s = include_str!("test_input2");
    assert_eq!(solve(s).1, 36);
}
