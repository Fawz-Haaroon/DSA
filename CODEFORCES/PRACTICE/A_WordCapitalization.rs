use std::io::{self, Read};

fn capitalize(word: &str) -> String {
    let mut bytes = word.as_bytes().to_vec();

    if bytes[0].is_ascii_lowercase() {
        bytes[0] = bytes[0].to_ascii_uppercase();
    }

    String::from_utf8(bytes).unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let word = input.trim();
    let result = capitalize(word);

    println!("{}", result);
}

