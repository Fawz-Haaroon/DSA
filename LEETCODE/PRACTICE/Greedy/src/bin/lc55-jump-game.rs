// LC55
// Jump Game
//
// Given an array where each element represents your maximum jump length,
// determine if you can reach the last index starting at index 0.

pub struct Solution {}


// SOLUTION
impl Solution {

    // GREEDY (REACHABILITY INVARIANT)  [ O(N) | O(1) ]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farthest: i32 = 0;
        let last = (nums.len() - 1) as i32;

        for (i, &jump) in nums.iter().enumerate() {
            let i = i as i32;

            if i > farthest {
                return false;
            }

            farthest = farthest.max(i + jump);

            if farthest >= last {
                return true;
            }
        }

        true
    }


    /*
    // DP (REDUNDANT)  [ O(N) | O(N) ]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![false; n];
        dp[0] = true;

        for i in 0..n {
            if !dp[i] { continue; }
            let max_jump = nums[i] as usize;
            for j in i+1..=usize::min(i + max_jump, n - 1) {
                dp[j] = true;
            }
        }

        dp[n - 1]
    }
    */


    /*
    // BACKTRACKING (TLE)  [ O(2^N) | O(N) ]
    */
}


// MAIN
fn main() {
    let nums1 = vec![2, 3, 1, 1, 4];
    let nums2 = vec![3, 2, 1, 0, 4];

    println!("Can jump (true): {}", Solution::can_jump(nums1));
    println!("Can jump (false): {}", Solution::can_jump(nums2));
}

