// LC153
// Find Minimum in Rotated Sorted Array
//
// The array was originally sorted in ascending order,
// then rotated at some pivot. No duplicates.
//
// Find the minimum element.

pub struct Solution {}


// SOLUTION
impl Solution {

    // BINARY SEARCH (INVARIANT-BASED)  [ O(log N) | O(1) ]
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0usize;
        let mut r = nums.len() - 1;

        while l < r {
            let mid = l + (r - l) / 2;

            if nums[mid] > nums[r] {
                // minimum must be in right half
                l = mid + 1;
            } else {
                // minimum is at mid or in left half
                r = mid;
            }
        }

        nums[l]
    }


    /*
    // LINEAR SCAN  [ O(N) | O(1) ]
    pub fn find_min(nums: Vec<i32>) -> i32 {
        *nums.iter().min().unwrap()
    }
    */
}


// MAIN
fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let ans = Solution::find_min(nums);
    println!("Minimum element: {}", ans);
}

