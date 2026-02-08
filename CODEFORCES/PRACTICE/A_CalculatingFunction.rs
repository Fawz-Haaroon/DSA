use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let n: i128 = s.trim().parse().unwrap();
    if n % 2 == 0 {
        println!("{}", n / 2);
    } else {
        println!("{}", -(n + 1) / 2);
    }
}
