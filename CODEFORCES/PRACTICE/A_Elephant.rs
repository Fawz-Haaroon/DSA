use std::io::{self, Read};

fn min_steps_to_reach(x: i32) -> i32 {
    (x + 4) / 5
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let x: i32 = input.trim().parse().unwrap();
    let result = min_steps_to_reach(x);

    println!("{}", result);
}

