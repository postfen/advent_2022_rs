use std::collections::HashMap;

pub fn print_solution() {
    let mut s = build_dirs(include_str!("input"));
    let s1 = q1(&s);
    let s2 = q2(&mut s);
    println!("--- Day 7: No Space Left On Device ---\nA. {s1}\nB. {s2}\n")
}

fn build_dirs(s: &str) -> HashMap<String, i32> {
    let mut dirs: HashMap<String, i32> = HashMap::new();
    let mut curr_dir = vec![];
    for line in s.lines() {
        let cmd = line.split_whitespace().collect::<Vec<&str>>();
        if cmd[0] == "$" {
            if cmd[1] == "cd" {
                if cmd[2] == ".." {
                    curr_dir.pop();
                } else {
                    curr_dir.push(cmd[2])
                }
            }
        } else if cmd[0].parse::<i64>().is_ok() {
            for i in 1..=curr_dir.len() {
                let path_str = curr_dir[1..i].join("/");
                *dirs.entry(path_str).or_insert(0) += cmd[0].parse::<i32>().unwrap();
            }
        }
    }
    dirs
}

fn q1(dirs: &HashMap<String, i32>) -> i32 {
    dirs.values().filter(|&&x| x <= 100000).sum()
}

fn q2(dirs: &mut HashMap<String, i32>) -> i32 {
    let space_free = 70000000i32 - dirs.get("").unwrap();
    let mut space_needed = 30000000i32 - space_free;
    dirs.retain(|_k, v| v >= &mut space_needed);
    let mut x = dirs.values().collect::<Vec<&i32>>();
    x.sort();

    *x[0]
}

#[cfg(test)]
#[test]
fn test_day5_1() {
    let s = build_dirs(include_str!("test_input"));
    let s1 = q1(&s);
    assert_eq!(s1, 95437);
}

#[test]
fn test_day5_2() {
    let mut s = build_dirs(include_str!("test_input"));
    let s2 = q2(&mut s);
    assert_eq!(s2, 24933642);
}
