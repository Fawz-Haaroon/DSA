pub struct Solution {}
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;


//SOLUTION
impl Solution {

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut count = HashMap::new();
        for n in nums {
            *count.entry(n).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        for (&num, &freq) in count.iter() {
            heap.push((Reverse(freq), num));
            if heap.len() as i32 > k { heap.pop(); }
        }

        heap.into_iter().map(|(_,num)| num).collect()
    }

}


//MAIN
fn main() {

    let nums = vec![1,2,1,2,1,4,3,4,5,4,4,1,4,2,3,1,3,2];
    let k = 9;

    let ans = Solution::top_k_frequent(nums, k);
    println!("Top {} frequent elements are: {:?}", k, ans)
}
