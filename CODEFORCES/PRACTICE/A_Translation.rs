use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let s = lines.next().unwrap().trim();
    let t = lines.next().unwrap().trim();

    let rev: String = s.chars().rev().collect();

    if rev == t {
        println!("YES");
    } else {
        println!("NO");
    }
}

