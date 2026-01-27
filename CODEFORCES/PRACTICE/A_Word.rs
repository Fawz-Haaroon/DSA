use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let s = s.trim();

    let mut upper = 0;
    let mut lower = 0;

    for c in s.chars() {
        if c.is_uppercase() { upper += 1; }
        else { lower += 1; }
    }

    if upper > lower {
        println!("{}", s.to_uppercase());
    } else {
        println!("{}", s.to_lowercase());
    }
}

