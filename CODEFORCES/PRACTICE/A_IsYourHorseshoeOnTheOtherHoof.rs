use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let nums: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let set: HashSet<_> = nums.iter().collect();
    println!("{}", 4 - set.len());
}
