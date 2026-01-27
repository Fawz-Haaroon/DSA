use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let k: i64 = it.next().unwrap().parse().unwrap();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let w: i64 = it.next().unwrap().parse().unwrap();

    let total = k * w * (w + 1) / 2;
    let borrow = (total - n).max(0);

    println!("{}", borrow);
}

