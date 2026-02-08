use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut res = vec![0usize; n];

    for i in 0..n {
        let p: usize = it.next().unwrap().parse().unwrap();
        res[p - 1] = i + 1;
    }

    for x in res {
        print!("{} ", x);
    }
}
