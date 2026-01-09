use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let mut scores = Vec::with_capacity(n);
    for _ in 0..n {
        scores.push(it.next().unwrap().parse::<i32>().unwrap());
    }

    let threshold = scores[k - 1];
    let mut count = 0;

    for &s in &scores {
        if s >= threshold && s > 0 {
            count += 1;
        }
    }

    println!("{}", count);
}

