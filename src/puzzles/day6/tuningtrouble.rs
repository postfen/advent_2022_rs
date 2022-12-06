use std::collections::HashSet;

pub fn print_solution() {
    let s = include_str!("input");
    let s1 = solve(s, 4);
    let s2 = solve(s, 14);
    println!("--- Day 6: Tuning Trouble ---\nA. {s1}\nB. {s2}\n")
}

fn solve(s: &str, size: usize) -> usize {
    s.chars().collect::<Vec<char>>().windows(size)
     .position(|x| HashSet::<char>::from_iter(x.iter().copied()).len() == size)
     .map(|x| x + size)
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
    let s1 = solve(test1, amt);
    let s2 = solve(test2, amt);
    let s3 = solve(test3, amt);
    let s4 = solve(test4, amt);
    assert_eq!(s1, 5);
    assert_eq!(s2, 6);
    assert_eq!(s3, 10);
    assert_eq!(s4, 11);
}

#[test]
fn test_day6_2() {
    let amt = 14;
    let test1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let test2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let test3 = "nppdvjthqldpwncqszvftbrmjlhg";
    let test4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let test5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let s1 = solve(test1, amt);
    let s2 = solve(test2, amt);
    let s3 = solve(test3, amt);
    let s4 = solve(test4, amt);
    let s5 = solve(test5, amt);
    assert_eq!(s1, 19);
    assert_eq!(s2, 23);
    assert_eq!(s3, 23);
    assert_eq!(s4, 29);
    assert_eq!(s5, 26);
    // let s = include_str!("test_input");
    // let s2 = solve(s).0;
}
