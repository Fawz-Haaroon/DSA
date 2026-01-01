pub struct Solution {}
use std::io::*;


//SOLUTION
impl Solution {

    // given constraint that ,the algorithm should run in O(N)
    // but first, thinking about the BF
    // its obvious the we sort the array, [O(N*logN)]
    // and use an incremental window to track the max sequence


    // BF attempt
    pub fn longest_consequtive(nums: Vec<i32>) -> i32 {

    }
}


//MAIN
fn main() {
    let nums = Vec![100,4,200,1,3,2];
    let ans = Solution::longest_consequtive(nums);
    println!("the size of the longest consecutive sequence is: {}",ans);
}
