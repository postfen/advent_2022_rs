pub fn print_solution() {
    let s =include_str!("test_input");
    let treemap = create_map(s);
    print_map(&treemap);
    let s1 = q1(s);

    let s2 = q2(s);
    println!("--- Day 8: Tree Top Treehouse ---\nA. {s1}\nB. {s2}\n")
}

fn create_map(s:&str)->Vec<&str>{
    s.trim().lines().collect()
}

fn print_map(treemap:&Vec<&str>){
    for r in treemap{
        for char in r.chars(){
            print!("{char}")
        }
        println!("");
    }
}

fn q1(s: &str) -> i32 {
    0
}

fn q2(s: &str) -> i32 {
    0
}

#[cfg(test)]
#[test]
fn test_day5_1() {
    let s = include_str!("test_input");
    let s1 = q1(&s);
    assert_eq!(s1, 16);
}

#[test]
fn test_day5_2() {
    let s = include_str!("test_input");
    let s2 = q2(&s);
    assert_eq!(s2, 95437);
}
