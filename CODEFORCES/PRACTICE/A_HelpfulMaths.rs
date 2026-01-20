use std::io::{self, Read};

fn reorder_sum(s: &str) -> String {
    let mut nums: Vec<char> = s
        .chars()
        .filter(|c| *c != '+')
        .collect();

    nums.sort_unstable();

    nums.into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("+")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let s = input.trim();
    let result = reorder_sum(s);

    println!("{}", result);
}

