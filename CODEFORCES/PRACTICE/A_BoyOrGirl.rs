use std::io::{self, Read};
use std::collections::HashSet;

fn gender_by_username(name: &str) -> &'static str {
    let distinct: HashSet<char> = name.chars().collect();

    if distinct.len() % 2 == 0 {
        "CHAT WITH HER!"
    } else {
        "IGNORE HIM!"
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let name = input.trim();
    let result = gender_by_username(name);

    println!("{}", result);
}

