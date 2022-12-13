
pub fn print_solution() {
    let s = include_str!("input");
    let s1 = q1(s);
    let s2 = q2(s);
    println!(
        "--- Day #: Title ---\n\
         A. {s1}\n\
         B. {s2}\n"
    );
}


#[cfg(test)]
#[test]
fn test_dayn_a() {
    let s = include_str!("test_input");
    assert_eq!(q1(s), 24000);
}
#[test]
fn test_dayn_b() {
    let s = include_str!("test_input");
    assert_eq!(q1(s), 45000);
}
