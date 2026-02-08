use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: f64 = it.next().unwrap().parse().unwrap();
    let mut sum = 0.0;

    for _ in 0..n as usize {
        let x: f64 = it.next().unwrap().parse().unwrap();
        sum += x;
    }

    println!("{}", sum / n);
}
