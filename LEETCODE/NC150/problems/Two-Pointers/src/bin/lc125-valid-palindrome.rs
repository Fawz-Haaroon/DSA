// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters
// and removing all non-alphanumeric characters, it reads the same forward and backward.

pub struct Solution {}


// SOLUTION
impl Solution {

    // TWO POINTER IN-PLACE  [ O(N) | O(1) ]
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut l: usize = 0;
        let mut r: usize = bytes.len().saturating_sub(1);

        while l < r {
            // move left pointer to next alphanumeric
            while l < r && !bytes[l].is_ascii_alphanumeric() { l += 1; }
            // move right pointer to previous alphanumeric
            while l < r && !bytes[r].is_ascii_alphanumeric() { r -= 1; }

            if l < r {
                if bytes[l].to_ascii_lowercase() != bytes[r].to_ascii_lowercase() {
                    return false;
                }
                l += 1;
                r = r.saturating_sub(1);
            }
        }

        true
    }



    // FILTER + REVERSE COMPARE  [ O(N) | O(N) ]
    pub fn is_palindrome(s: String) -> bool {
        let filtered: Vec<u8> = s
            .bytes()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let mut i = 0;
        let mut j = filtered.len().saturating_sub(1);
        while i < j {
            if filtered[i] != filtered[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }




    // BRUTE FORCE  [ O(N) | O(N) ]
    pub fn is_palindrome(s: String) -> bool {
        let mut t = String::new();
        for c in s.chars() {
            if c.is_alphanumeric() {
                t.push(c.to_ascii_lowercase());
            }
        }
        let rev: String = t.chars().rev().collect();
        t == rev
    }


}


// MAIN
fn main() {
    let s = String::from("A man, a plan, a canal: Panama");
    let ans = Solution::is_palindrome(s);
    println!("Valid Palindrome? : {}", ans);
}
