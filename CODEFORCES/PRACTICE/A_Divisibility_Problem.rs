use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();
        let r = a % b;
        if r == 0 {
            out.push_str("0\n");
        } else {
            out.push_str(&(b - r).to_string());
            out.push('\n');
        }
    }

    print!("{}", out);
}
