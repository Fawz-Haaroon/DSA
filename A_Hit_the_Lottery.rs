use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut n: i64 = input.trim().parse().unwrap();

    let bills = [100, 20, 10, 5, 1];
    let mut cnt = 0;

    for &b in &bills {
        cnt += n / b;
        n %= b;
    }

    println!("{}", cnt);
}
