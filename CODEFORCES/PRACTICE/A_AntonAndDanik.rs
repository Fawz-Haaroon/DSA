use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let _n: usize = lines.next().unwrap().trim().parse().unwrap();
    let s = lines.next().unwrap().trim();

    let mut a = 0;
    let mut d = 0;

    for c in s.chars() {
        if c == 'A' { a += 1; }
        else { d += 1; }
    }

    if a > d {
        println!("Anton");
    } else if d > a {
        println!("Danik");
    } else {
        println!("Friendship");
    }
}

