use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();

    let mut out = String::new();
    for i in 1..=n {
        if i % 2 == 1 {
            out.push_str("I hate");
        } else {
            out.push_str("I love");
        }
        if i < n {
            out.push_str(" that ");
        }
    }
    out.push_str(" it");
    println!("{}", out);
}
