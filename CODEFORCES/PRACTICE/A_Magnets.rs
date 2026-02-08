use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.lines();
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut prev = String::new();
    let mut groups = 0;

    for _ in 0..n {
        let cur = it.next().unwrap().to_string();
        if cur != prev {
            groups += 1;
            prev = cur;
        }
    }
    println!("{}", groups);
}
