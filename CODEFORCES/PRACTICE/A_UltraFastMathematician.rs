use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.lines();

    let a = it.next().unwrap().as_bytes();
    let b = it.next().unwrap().as_bytes();

    let res: String = a.iter()
        .zip(b.iter())
        .map(|(x, y)| if x == y { '0' } else { '1' })
        .collect();

    println!("{}", res);
}
