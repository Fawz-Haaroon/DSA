use std::io::{self, Read};

fn pattern_x(n: usize) {
    for _ in 0..n {
        for _ in 0..n {
            print!("*");
        }
        println!("");
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let n: usize = input
        .trim()
        .parse()
        .expect("Failed to parse input as an unsigned integer.");

    if n == 0 {
        eprintln!("Error: n must be greater than 0.");
        std::process::exit(1);
    }

    pattern_x(n);
}
