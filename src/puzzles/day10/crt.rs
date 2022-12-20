pub fn print_solution() {
    let s = include_str!("input");
    let (s1, s2) = solve(s);
    println!( "--- Day 10: Cathode Ray Tube ---\nA. {s1}\nB.\n{s2}\n");
}

fn solve(s: &str) -> (i32, String) {
    let mut x = 1;
    let mut crt_output = String::new();
    let mut beam_pos = 0;
    let mut cycle_count = 0;
    let mut cycles: Vec<i32> = Vec::new();
    let mut temp_cycle;
    let mut add_amt;
    for line in s.lines() {
        let signal = line.chars().next().unwrap();
        match signal {
            'n' => { //noop
                add_amt = 0;
                temp_cycle = 1;
            }
            'a' => { //addx
                temp_cycle = 2;
                add_amt = line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            }
            _ => panic!("Invalid Signal"),
        }
        for _ in 0..temp_cycle {
            cycle_count += 1;
            if [x - 1, x, x + 1].contains(&beam_pos) {
                crt_output.push('#');
            } else {
                crt_output.push('.');
            }
            if [20, 60, 100, 140, 180, 220].contains(&cycle_count) { //Part 1
                cycles.push(cycle_count * x);
            }
            beam_pos += 1;
            if beam_pos == 40 {
                crt_output.push('\n');
                beam_pos = 0;
            }
        }
        x += add_amt;
    }
    (cycles.iter().sum(), crt_output)
}

#[cfg(test)]
#[test]
fn test_dayn_a() {
    let s = include_str!("test_input2");
    assert_eq!(solve(s).0, 13140);
}

