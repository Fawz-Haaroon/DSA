use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let s = s.trim();

    let mut cnt = 0;
    for c in s.chars() {
        if c == '4' || c == '7' {
            cnt += 1;
        }
    }

    let lucky = cnt > 0 && cnt.to_string().chars().all(|c| c == '4' || c == '7');

    if lucky {
        println!("YES");
    } else {
        println!("NO");
    }
}

