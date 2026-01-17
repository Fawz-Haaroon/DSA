// LC229
// Majority Element II
//
// Find all elements appearing more than ⌊ n / 3 ⌋ times.
// There can be at most TWO such elements.

pub struct Solution {}


// SOLUTION
impl Solution {

    // BOYER–MOORE (n/3 GENERALIZATION)  [ O(N) | O(1) ]
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut cand1 = 0;
        let mut cand2 = 0;
        let mut cnt1 = 0;
        let mut cnt2 = 0;

        // voting phase
        for &x in &nums {
            if cnt1 > 0 && x == cand1 {
                cnt1 += 1;
            } else if cnt2 > 0 && x == cand2 {
                cnt2 += 1;
            } else if cnt1 == 0 {
                cand1 = x;
                cnt1 = 1;
            } else if cnt2 == 0 {
                cand2 = x;
                cnt2 = 1;
            } else {
                cnt1 -= 1;
                cnt2 -= 1;
            }
        }

        // verification phase
        let mut res = Vec::new();
        let n = nums.len() as i32;
        let mut c1 = 0;
        let mut c2 = 0;

        for &x in &nums {
            if x == cand1 { c1 += 1; }
            else if x == cand2 { c2 += 1; }
        }

        if c1 > n / 3 { res.push(cand1); }
        if c2 > n / 3 { res.push(cand2); }

        res
    }


    /*
    // HASHMAP COUNTING  [ O(N) | O(N) ]
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let n = nums.len() as i32;

        for x in nums {
            *map.entry(x).or_insert(0) += 1;
        }

        map.into_iter()
           .filter(|(_, v)| *v > n / 3)
           .map(|(k, _)| k)
           .collect()
    }
    */


    /*
    // BRUTE FORCE  [ O(N^2) | O(1) ]
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let n = nums.len();

        for i in 0..n {
            if res.contains(&nums[i]) { continue; }
            let mut cnt = 0;
            for j in 0..n {
                if nums[i] == nums[j] {
                    cnt += 1;
                }
            }
            if cnt > n / 3 {
                res.push(nums[i]);
            }
        }
        res
    }
    */
}


// MAIN
fn main() {
    let nums = vec![1, 2, 3, 1, 2, 1, 1];
    let ans = Solution::majority_element(nums);
    println!("Majority elements (> n/3): {:?}", ans);
}

