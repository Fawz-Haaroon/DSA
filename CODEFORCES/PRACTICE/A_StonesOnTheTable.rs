use std::io::{self, Read};

fn stones_to_remove(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut count = 0;

    for i in 1..bytes.len() {
        if bytes[i] == bytes[i - 1] {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _n: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap();

    let ans = stones_to_remove(s);
    println!("{}", ans);
}

