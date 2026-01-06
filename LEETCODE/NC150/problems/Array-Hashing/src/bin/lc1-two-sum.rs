pub struct Solution {}
use std::collections::HashMap;

// SOLUTION
impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, val) in nums.iter().enumerate() {
            let comp = target - *val;

            if let Some(&j) = map.get(&comp) { return vec![j as i32, i as i32]; }
            else { map.insert(*val, i); }
        }

        vec![]
    }
}

// MAIN
fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target = 9;

    let ans = Solution::two_sum(nums, target);
    println!("indices of nums that add up to target: {:?}", ans);
}

