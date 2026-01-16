// LC2943
// Maximize Area of Square Hole in Grid
//
// Removing consecutive bars creates merged continuous gaps.
// The largest square side is determined by the minimum of the
// largest horizontal gap and vertical gap.

pub struct Solution {}


// SOLUTION
impl Solution {

    // SORT + LONGEST CONSECUTIVE RUN  [ O(N log N) | O(1) ]
    pub fn maximize_square_hole_area(
        _n: i32,
        _m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {

        h_bars.sort_unstable();
        v_bars.sort_unstable();

        let max_h = Self::max_consecutive(&h_bars);
        let max_v = Self::max_consecutive(&v_bars);

        let side = (max_h + 1).min(max_v + 1);
        side * side
    }

    #[inline]
    fn max_consecutive(arr: &Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }

        let mut best = 1;
        let mut cur = 1;

        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] + 1 {
                cur += 1;
                best = best.max(cur);
            } else {
                cur = 1;
            }
        }

        best
    }


    /*
    // HASHSET + SCAN  [ O(N) | O(N) ]
    pub fn maximize_square_hole_area(
        _n: i32,
        _m: i32,
        h_bars: Vec<i32>,
        v_bars: Vec<i32>,
    ) -> i32 {
        use std::collections::HashSet;

        let hs: HashSet<i32> = h_bars.iter().copied().collect();
        let vs: HashSet<i32> = v_bars.iter().copied().collect();

        let max_h = Self::scan(&hs);
        let max_v = Self::scan(&vs);

        let side = (max_h + 1).min(max_v + 1);
        side * side
    }

    fn scan(set: &HashSet<i32>) -> i32 {
        let mut best = 0;
        for &x in set {
            if !set.contains(&(x - 1)) {
                let mut cur = 1;
                let mut y = x + 1;
                while set.contains(&y) {
                    cur += 1;
                    y += 1;
                }
                best = best.max(cur);
            }
        }
        best
    }
    */
}


// MAIN
fn main() {
    let n = 2;
    let m = 1;
    let h_bars = vec![2, 3];
    let v_bars = vec![2];

    let ans = Solution::maximize_square_hole_area(n, m, h_bars, v_bars);
    println!("Max square area: {}", ans);
}

