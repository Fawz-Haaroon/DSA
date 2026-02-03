use std::io::{self, Read};

fn count_available_rooms(pairs: &[(i32, i32)]) -> i32 {
    pairs.iter().filter(|(p, q)| q - p >= 2).count() as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut rooms = Vec::with_capacity(n);

    for _ in 0..n {
        let p: i32 = it.next().unwrap().parse().unwrap();
        let q: i32 = it.next().unwrap().parse().unwrap();
        rooms.push((p, q));
    }

    let ans = count_available_rooms(&rooms);
    println!("{ans}");
}

