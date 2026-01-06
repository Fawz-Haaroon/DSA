pub struct Solution {}
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;


// SOLUTION
impl Solution {


    // HashSET [ O(N) | O(N) ]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        for n in nums {
            if !seen.insert(n) {
                return true;
            }
        }
        false
    }


    // HashMAP [ O(N) | O(N) ]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();
        for n in nums {
            *count.entry(n).or_insert(0) += 1;
            if count[&n] > 1 {
    return true;
            }
        }
        false
    }


    // Adjacent-Check  [ O(N * logN) | O(1) ]
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return true;
            }
        }
        false
    }


    // Sort + deduplication  [ O(N * logN) | O(1) ]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let x = nums.len();
        nums.sort();
        nums.dedup();
        let y = nums.len();
        x != y
    }


    // Brute Force [ O(N^2) | O(1) ]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }


}


// MAIN
fn main() {
    let nums = vec![1, 2, 3, 1];
    let ans = Solution::contains_duplicate(nums);
    println!("Contains duplicate? : {}", ans);
}


