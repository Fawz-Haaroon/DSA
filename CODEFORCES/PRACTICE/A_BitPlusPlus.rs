use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut x = 0i32;

    for _ in 0..n {
        let s = it.next().unwrap();
        if s.contains("++") {
            x += 1;
        } else {
            x -= 1;
        }
    }

    println!("{}", x);
}

