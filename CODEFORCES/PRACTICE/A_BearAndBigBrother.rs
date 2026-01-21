use std::io::{self, Read};

fn years_until_larger(mut a: i32, mut b: i32) -> i32 {
    let mut years = 0;

    while a <= b {
        a *= 3;
        b *= 2;
        years += 1;
    }

    years
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    let result = years_until_larger(a, b);
    println!("{}", result);
}

