use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();

    let k = nums[0];
    let l = nums[1];
    let m = nums[2];
    let n = nums[3];
    let d = nums[4];

    let mut cnt = 0;
    for i in 1..=d {
        if i % k == 0 || i % l == 0 || i % m == 0 || i % n == 0 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
