use std::io::{self, Read};

fn can_divide(w: i32) -> bool {
    w > 2 && w % 2 == 0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let w: i32 = it.next().unwrap().parse().unwrap();

    let ans = can_divide(w);

    if ans {
        println!("YES");
    } else {
        println!("NO");
    }
}

