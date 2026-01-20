use std::io::{self, Read};

fn compare_case_insensitive(a: &str, b: &str) -> i32 {
    let a = a.to_ascii_lowercase();
    let b = b.to_ascii_lowercase();

    match a.cmp(&b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Equal => 0,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let s1 = it.next().unwrap();
    let s2 = it.next().unwrap();

    let res = compare_case_insensitive(s1, s2);
    println!("{}", res);
}

