pub fn print_solution() {
    let s = include_str!("input");
    let (a1, a2) = solve(&s);
    println!("--- Day 4: Camp Cleanup ---\nA.  {a1}\nB.  {a2}\n")}

fn solve(s: &str) -> (i32, i32) {
    let mut q1_total = 0;
    let mut q2_total = 0;
    for line in s.lines() {
        let r = line.split(['-', ',']).filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<i64>>();
        if (r[0] <= r[2] && r[1] >= r[3]) | (r[2] <= r[0] && r[3] >= r[1]) {
            q1_total += 1
        }
        if ((r[0]..=r[1]).contains(&r[2])) | (r[0]..=r[1]).contains(&r[3])
         | ((r[2]..=r[3]).contains(&r[0]) | ((r[2]..r[3] + 1).contains(&r[0]))){
            q2_total += 1
        }
    }
    (q1_total, q2_total)
}
