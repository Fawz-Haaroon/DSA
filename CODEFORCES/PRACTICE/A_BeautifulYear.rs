use std::io::{self, Read};

fn distinct(mut y: i32) -> bool {
    let mut seen = [false; 10];
    while y > 0 {
        let d = (y % 10) as usize;
        if seen[d] {
            return false;
        }
        seen[d] = true;
        y /= 10;
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut y: i32 = input.trim().parse().unwrap();

    loop {
        y += 1;
        if distinct(y) {
            println!("{}", y);
            break;
        }
    }
}

