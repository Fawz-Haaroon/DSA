use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let a: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();

    let mut max_i = 0;
    let mut min_i = 0;

    for i in 0..n {
        if a[i] > a[max_i] {
            max_i = i;
        }
        if a[i] <= a[min_i] {
            min_i = i;
        }
    }

    let mut ans = max_i + (n - 1 - min_i);
    if max_i > min_i {
        ans -= 1;
    }

    println!("{}", ans);
}
