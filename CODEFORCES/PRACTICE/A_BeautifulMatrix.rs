use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut row: i32 = 0;
    let mut col: i32 = 0;

    for i in 1..=5 {
        for j in 1..=5 {
            let val: i32 = it.next().unwrap().parse().unwrap();
            if val == 1 {
                row = i;
                col = j;
            }
        }
    }

    let moves = (row - 3).abs() + (col - 3).abs();
    println!("{}", moves);
}

