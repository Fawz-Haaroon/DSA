use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut cur = 0;
    let mut max_cap = 0;

    for _ in 0..n {
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();

        cur -= a;
        cur += b;

        if cur > max_cap {
            max_cap = cur;
        }
    }

    println!("{}", max_cap);
}

