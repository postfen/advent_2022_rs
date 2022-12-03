use std::collections::HashSet;
use std::ops::BitAnd;
use std::{
    fs::File,
    io::{self, Read},
};

trait Value {
    fn value(&self) -> u8;
}

impl Value for char {
    fn value(&self) -> u8 {
        if self.is_lowercase() {
            return *self as u8 - b'`'; // a -> 1
        } else {
            return *self as u8 - b'&'; // A -> 27
        }
    }
}

fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

fn question_1(s: &String) {
    let mut total: i32 = 0;
    for line in s.lines() {
        let (a, b) = &line.split_at(line.len() / 2);
        let a_set: HashSet<char> = a.chars().collect();
        let b_set: HashSet<char> = b.chars().collect();
        total += a_set.bitand(&b_set).into_iter().next().unwrap().value() as i32;
    }
    println!("{}", total)
}

fn question_2(s: &String) {
    let mut total: i32 = 0;
    let mut lines = s.lines();
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let a: HashSet<char> = a.chars().collect();
        let b: HashSet<char> = b.chars().collect();
        let c: HashSet<char> = c.chars().collect();
        total += c.bitand(&a.bitand(&b)).into_iter().next().unwrap().value() as i32;
    }
    println!("{}", total)
}


fn main() {
    let s = read_file_to_string("input").unwrap_or_default();
    question_1(&s);
    question_2(&s);
}
