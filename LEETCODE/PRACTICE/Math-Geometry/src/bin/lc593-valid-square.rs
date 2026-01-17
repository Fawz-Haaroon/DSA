// LC593
// Valid Square
//
// A square has:
// - 4 equal sides (positive length)
// - 2 equal diagonals
// Order of points is arbitrary.

pub struct Solution {}


// SOLUTION
impl Solution {

    // DISTANCE MULTISET CHECK  [ O(1) | O(1) ]
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut d = Vec::with_capacity(6);

        d.push(Self::dist2(&p1, &p2));
        d.push(Self::dist2(&p1, &p3));
        d.push(Self::dist2(&p1, &p4));
        d.push(Self::dist2(&p2, &p3));
        d.push(Self::dist2(&p2, &p4));
        d.push(Self::dist2(&p3, &p4));

        d.sort_unstable();

        // 4 equal sides + 2 equal diagonals
        d[0] > 0 &&
        d[0] == d[1] &&
        d[1] == d[2] &&
        d[2] == d[3] &&
        d[4] == d[5]
    }

    #[inline]
    fn dist2(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
        let dx = a[0] - b[0];
        let dy = a[1] - b[1];
        dx * dx + dy * dy
    }


    /*
    // VECTOR + DOT PRODUCT CHECK  [ O(1) | O(1) ]
    // Harder to reason about, more fragile in interviews.
    */
}


// MAIN
fn main() {
    let p1 = vec![0, 0];
    let p2 = vec![1, 1];
    let p3 = vec![1, 0];
    let p4 = vec![0, 1];

    let ans = Solution::valid_square(p1, p2, p3, p4);
    println!("Valid square? {}", ans);
}

