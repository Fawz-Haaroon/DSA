use std::io::{self, Read};

fn abbreviate(word: &str) -> String {
    let len = word.len();

    if len <= 10 {
        return word.to_string();
    }

    let first = word.chars().next().unwrap();
    let last = word.chars().last().unwrap();
    let middle_count = len - 2;

    format!("{}{}{}", first, middle_count, last)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let word = it.next().unwrap();
        let result = abbreviate(word);
        println!("{}", result);
    }
}

