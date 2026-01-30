use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut hard = false;

    for _ in 0..n {
        let x: i32 = it.next().unwrap().parse().unwrap();
        if x == 1 {
            hard = true;
        }
    }

    if hard {
        println!("HARD");
    } else {
        println!("EASY");
    }
}

