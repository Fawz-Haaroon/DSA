// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that
// i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
// The solution set must not contain duplicate triplets.

pub struct Solution {}


// SOLUTION
impl Solution {

    // TWO POINTER AFTER SORT  [ O(N^2) | O(1) ]
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();
        if n < 3 { return res; }

        nums.sort_unstable();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate fixed element
            }

            let mut l = i + 1;
            let mut r = n - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];

                if sum == 0 {
                    res.push(vec![nums[i], nums[l], nums[r]]);

                    // skip duplicates for left
                    while l < r && nums[l] == nums[l + 1] { l += 1; }
                    // skip duplicates for right
                    while l < r && nums[r] == nums[r - 1] { r -= 1; }

                    l += 1;
                    r -= 1;
                }
                else if sum < 0 {
                    l += 1;
                }
                else {
                    r -= 1;
                }
            }
        }

        res
    }



    // HASHSET FIXED i + TWO SUM VARIANT  [ O(N^2) | O(N) ]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut seen: HashSet<(i32,i32,i32)> = HashSet::new();
        let n = nums.len();

        for i in 0..n {
            let mut set = HashSet::new();
            for j in i+1..n {
                let target = -nums[i] - nums[j];
                if set.contains(&target) {
                    let mut triplet = vec![nums[i], nums[j], target];
                    triplet.sort_unstable();
                    let key = (triplet[0], triplet[1], triplet[2]);
                    if seen.insert(key) {
                        res.push(triplet);
                    }
                }
                set.insert(nums[j]);
            }
        }

        res
    }




    // BRUTE FORCE  [ O(N^3) | O(1) ]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let n = nums.len();

        for i in 0..n {
            for j in i+1..n {
                for k in j+1..n {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut t = vec![nums[i], nums[j], nums[k]];
                        t.sort_unstable();
                        if !res.contains(&t) {
                            res.push(t);
                        }
                    }
                }
            }
        }
        res
    }


}


// MAIN
fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let ans = Solution::three_sum(nums);
    println!("3Sum Triplets: {:?}", ans);
}

