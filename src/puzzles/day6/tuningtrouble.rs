use std::collections::HashSet;

pub fn print_solution() {
    let s = include_str!("input");
    let s1 = solve(s, 4);
    let s2 = solve(s, 14);
    println!("--- Day 6: Tuning Trouble ---\nA. {s1}\nB. {s2}\n");
}

fn solve(s: &str, size: usize) -> usize {
    s.chars().collect::<Vec<char>>().windows(size)
     .position(|w| w.iter().copied().collect::<HashSet<char>>().len() == size)
     .map(|i| i + size) // get last position in window
     .unwrap()
}

#[cfg(test)]
#[test]
fn test_day6_1() {
    let amt = 4;
    let test1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let test2 = "nppdvjthqldpwncqszvftbrmjlhg";
    let test3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let test4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(solve(test1, amt), 5);
    assert_eq!(solve(test2, amt), 6);
    assert_eq!(solve(test3, amt), 10);
    assert_eq!(solve(test4, amt), 11);
}

#[test]
fn test_day6_2() {
    let amt = 14;
    let test1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let test2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let test3 = "nppdvjthqldpwncqszvftbrmjlhg";
    let test4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let test5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(solve(test1, amt), 19);
    assert_eq!(solve(test2, amt), 23);
    assert_eq!(solve(test3, amt), 23);
    assert_eq!(solve(test4, amt), 29);
    assert_eq!(solve(test5, amt), 26);
    // let s = include_str!("test_input");
    // let s2 = solve(s).0;
}
