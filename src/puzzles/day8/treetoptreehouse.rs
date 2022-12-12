pub fn print_solution() {
    let s = include_str!("test_input");
    let treemap = create_map(s);
    get_border_debug(&treemap);
    let s1 = q1(s);
    let s2 = q2(s);
    println!("--- Day 8: Tree Top Treehouse ---\nA. {s1}\nB. {s2}\n");
}

fn create_map(s: &str) -> Vec<&str> {
    s.trim().lines().collect()
}

fn get_border_debug(treemap: &[&str]) {
    let length = treemap[0].len();
    let height = treemap.len();
    println!("Length: {}\nHeight: {}", length, height);
    println!(
        "Trees that can be seen from the outside: {}",
        (length * 2 + height * 2) - 4
    );
}

fn count_visible(treemap: &[&str]) -> usize {
    0
}

fn get_boundaries(treemap: &[&str]) -> [usize; 4] {
    let length = treemap[0].len();
    let height = treemap.len();
    [0, 0, length, height]
}


fn q1(s: &str) -> usize{
    let treemap = create_map(s);
    let bounds = get_boundaries(&treemap);
    let mut n = 0;
    for (i, r) in treemap.iter().enumerate() {
        for (j, c) in r.chars().enumerate() {
            if bounds.contains(&j) | bounds.contains(&i) {
                n+=1;
            }
        }
    }
    n
}

fn q2(s: &str) -> i32 {
    s.parse().unwrap_or_default()
}

#[cfg(test)]
#[test]
fn test_day5_1() {
    print_solution();
    let s = include_str!("test_input");
    let s1 = q1(s);
    assert_eq!(s1, 16);
}

#[test]
fn test_day5_2() {
    let s = include_str!("test_input");
    let s2 = q2(s);
    assert_eq!(s2, 95437);
}
