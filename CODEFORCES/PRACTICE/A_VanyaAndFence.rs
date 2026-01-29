use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let h: i32 = it.next().unwrap().parse().unwrap();

    let mut width = 0;
    for _ in 0..n {
        let a: i32 = it.next().unwrap().parse().unwrap();
        if a > h {
            width += 2;
        } else {
            width += 1;
        }
    }

    println!("{}", width);
}

