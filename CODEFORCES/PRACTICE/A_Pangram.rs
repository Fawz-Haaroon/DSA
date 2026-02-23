use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _n: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap().to_lowercase();

    let mut seen = [false; 26];
    for c in s.bytes() {
        seen[(c - b'a') as usize] = true;
    }

    if seen.iter().all(|&x| x) {
        println!("YES");
    } else {
        println!("NO");
    }
}
