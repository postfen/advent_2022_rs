use core::fmt;
use std::cmp::Ordering;

pub fn print_solution() {
    let s = include_str!("input");
    let mut treemap = Treemap::from(s);
    let solutions = solve(&mut treemap);

    println!(
        "--- Day 8: Tree Top Treehouse ---\nA. {}\nB. {}\n",
        solutions.0, solutions.1
    );
}

#[derive(Debug)]
struct Treemap {
    grid: Vec<Vec<u32>>,
    bound_x: usize,
    bound_y: usize,
    scenic_score: usize,
}

impl Treemap {
    pub fn from(s: &str) -> Self {
        let s = s
            .trim()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let length = s[0].len() - 1;
        let height = s.len() - 1;

        Treemap {
            grid: s,
            bound_x: length,
            bound_y: height,
            scenic_score: 0,
        }
    }
}

impl fmt::Display for Treemap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid_str = String::new();
        for r in &self.grid {
            for c in r {
                grid_str += &c.to_string();
            }
            grid_str += "\n";
        }
        write!(f, "{}", grid_str)
    }
}
// fn visible_from_outside(tree: Tree, treemap: &Treemap) -> bool {
fn visible_from_outside(tree: u32, col: usize, row: usize, treemap: &Treemap) -> bool {
    let mut seen = (0, 0, 0, 0); // up, down, left, right
                                 // println!("N: ({}, {} : {})", row, col, tree );
                                 // Up
    for n in 0..row {
        if treemap.grid[n][col] >= tree {
            seen.0 += 1;
            break;
        }
    }
    // Down
    for n in row + 1..=treemap.bound_y {
        if treemap.grid[n][col] >= tree {
            seen.1 += 1;
            break;
        }
    }

    // Left
    for n in 0..col {
        if treemap.grid[row][n] >= tree {
            seen.2 += 1;
            break;
        }
    }

    // Right
    for n in col + 1..=treemap.bound_x {
        if treemap.grid[row][n] >= tree {
            seen.3 += 1;
            break;
        }
    }

    // println!("N: {:?}", seen);
    [seen.0, seen.1, seen.2, seen.3].contains(&0)
}

fn calculate_scenic_score(tree: u32, col: usize, row: usize, treemap: &Treemap) -> usize {
    let mut seen = [0, 0, 0, 0]; // up, down, left, right

    // Up
    for n in (0..row).rev() {
        match tree.cmp(&treemap.grid[n][col]) {
            Ordering::Less | Ordering::Equal => {
                seen[0] += 1;
                break;
            }
            Ordering::Greater => seen[0] += 1,
        }
    }

    // Down
    for n in row + 1..=treemap.bound_y {
        match tree.cmp(&treemap.grid[n][col]) {
            Ordering::Less | Ordering::Equal => {
                seen[1] += 1;
                break;
            }
            Ordering::Greater => seen[1] += 1,
        }
    }

    // Left
    for n in (0..col).rev() {
        match tree.cmp(&treemap.grid[row][n]) {
            Ordering::Less | Ordering::Equal => {
                seen[2] += 1;
                break;
            }
            Ordering::Greater => seen[2] += 1,
        }
    }

    // Right
    for n in col + 1..=treemap.bound_x {
        match tree.cmp(&treemap.grid[row][n]) {
            Ordering::Less | Ordering::Equal => {
                seen[3] += 1;
                break;
            }
            Ordering::Greater => seen[3] += 1,
        }
    }

    let scenic_score: usize = seen.iter().product();

    if scenic_score > treemap.scenic_score {
        return scenic_score;
    }
    treemap.scenic_score
}

fn solve(treemap: &mut Treemap) -> (u32, u32) {
    let mut trees_visible = 0;
    for (row, line) in treemap.grid.iter().enumerate() {
        for (col, tree) in line.iter().enumerate() {
            treemap.scenic_score = calculate_scenic_score(*tree, col, row, treemap);
            if col == 0 || col == treemap.bound_x || row == 0 || row == treemap.bound_y {
                trees_visible += 1;
                continue;
            } else if visible_from_outside(*tree, col, row, treemap) {
                trees_visible += 1;
            }
        }
    }
    (trees_visible, treemap.scenic_score.try_into().unwrap())
}

#[cfg(test)]
#[test]
fn test_day5_1() {
    print_solution();
    let s = include_str!("test_input");
    let mut treemap = Treemap::from(s);
    let s1 = solve(&mut treemap).0;
    assert_eq!(s1, 21);
}

#[test]
fn test_day5_2() {
    let s = include_str!("test_input");
    let mut treemap = Treemap::from(s);
    let s2 = solve(&mut treemap).1;
    assert_eq!(s2, 8);
}
