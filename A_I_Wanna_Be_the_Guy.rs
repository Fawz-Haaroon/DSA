use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let p: usize = it.next().unwrap().parse().unwrap();

    let mut can = vec![false; n + 1];

    for _ in 0..p {
        let x: usize = it.next().unwrap().parse().unwrap();
        can[x] = true;
    }

    let q: usize = it.next().unwrap().parse().unwrap();
    for _ in 0..q {
        let x: usize = it.next().unwrap().parse().unwrap();
        can[x] = true;
    }

    if (1..=n).all(|i| can[i]) {
        println!("I become the guy.");
    } else {
        println!("Oh, my keyboard!");
    }
}
