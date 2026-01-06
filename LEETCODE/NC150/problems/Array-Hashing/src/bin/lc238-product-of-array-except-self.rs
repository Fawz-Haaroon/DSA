pub struct Solution {}

//SOLUTION
impl Solution {



    // CASE-HANDLING OPTIMIZATION
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

        let n = nums.len();
        let mut res: Vec<i32> = vec![0; n];
        let zero_ct = nums.iter().filter(|&&x| x == 0).count();

        if zero_ct == 1 {
            let zero_idx = nums.iter().position(|&x| x == 0).unwrap();
            let product_val: i32 = nums.iter().filter(|&&x| x != 0).product();
            res[zero_idx] = product_val;
            return res;
        }

        if zero_ct == 0 {

            res = vec![1; n];
            // prefix pass
            let mut prefix = 1;
            for i in 0..n {
                res[i] = prefix;
                prefix *= nums[i];
            }

            // suffix pass
            let mut suffix = 1;
            for i in (0..n).rev() {
                res[i] *= suffix;
                suffix *= nums[i];
            }
        }

        res
    }


    // BRUTE-FORCE [ O(N*N) | O(N)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        (0..n)
            .map(|i| nums.iter().enumerate()
                          .filter(|&(j, _)| j != i)
                          .map(|(_, &v)| v)
                          .product()
            ).collect()
    }

}

//MAIN
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let res = Solution::product_except_self(nums);
    println!("{:?}", res);
}

