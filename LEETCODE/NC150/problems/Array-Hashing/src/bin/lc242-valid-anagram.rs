// An anagram is a word or phrase formed by rearranging the letters of a different word or phrase, using all the original letters exactly once.
pub struct Solution {}
use std::collections::HashMap;


// SOLUTION
impl Solution {


    // FIXED-SIZE  [ O(N) | O(1)<const 26> ]
    pub fn is_anagram(s: &str, t: &str) -> bool {
        if s.len() != t.len() { return false; }
        let mut freq = [0; 26];

        for c in s.bytes() {
            freq[(c - b'a') as usize] += 1;
        }
        for c in t.bytes() {
            freq[(c - b'a') as usize] -= 1;
        }
        //after ++ and -- end result must be 0
        freq.iter().all(|&x| x == 0)
    }


    // HASH N COMPARE [ O(N) | O(N) ]
    pub fn is_anagram(s: &str, t: &str) -> bool{
        let mut count1 = HashMap::new();
        for n in s.chars() { *count1.entry(n).or_insert(0) += 1; }
        let mut count2 = HashMap::new();
        for n in t.chars() { *count2.entry(n).or_insert(0) += 1; }
        // we can also do this in single pass by incrementing and decrementing the same `count` through 's' and 't'
        // count.values().all(|&v| v == 0)
        count1 == count2
    }


    // SORT N COMPARE [ O(N logN) | O(N) ]
    pub fn is_anagram(s: &str, t: &str) -> bool {
        if s.len() != t.len() { return false; }

        let mut a: Vec<char> = s.chars().collect();
        let mut b: Vec<char> = t.chars().collect();

        a.sort_unstable();
        b.sort_unstable();

        a == b
    }


    // BRUTE FORCE [ O(N^2) | O(1) ]
    pub fn is_anagram(s: &str, t: &str) -> bool {
        if s.len() != t.len() { return false; }

        let mut a: Vec<char> = s.chars().collect();
        let mut b: Vec<char> = t.chars().collect();

        // manual bubble sort / selection sort idea
        for i in 0..a.len() {
            for j in i+1..a.len() {
                if a[j] < a[i] { a.swap(i, j); }
                if b[j] < b[i] { b.swap(i, j); }
            }
        }
        a == b
    }

}



// MAIN
fn main() {
    let s = "anagram";
    let t = "nagaram";

    let ans = Solution::is_anagram(s,t);
    println!("Valid Anagram? : {}", ans);

}
