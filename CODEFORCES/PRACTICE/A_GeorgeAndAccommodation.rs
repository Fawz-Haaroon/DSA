use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut count = 0;

    for _ in 0..n {
        let p: i32 = it.next().unwrap().parse().unwrap();
        let q: i32 = it.next().unwrap().parse().unwrap();
        if q - p >= 2 {
            count += 1;
        }
    }

    println!("{}", count);
}
