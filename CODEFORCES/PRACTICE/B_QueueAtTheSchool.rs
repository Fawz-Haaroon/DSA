use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let t: usize = it.next().unwrap().parse().unwrap();
    let mut s: Vec<char> = it.next().unwrap().chars().collect();

    for _ in 0..t {
        let mut i = 0;
        while i + 1 < n {
            if s[i] == 'B' && s[i + 1] == 'G' {
                s.swap(i, i + 1);
                i += 2;
            } else {
                i += 1;
            }
        }
    }

    let result: String = s.into_iter().collect();
    println!("{}", result);
}

