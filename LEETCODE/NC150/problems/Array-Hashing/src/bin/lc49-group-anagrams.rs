pub struct Solution {}
use std::collections::HashMap;


// SOLUTION
impl Solution {


/// GROUPS ANAGRAMS USING is_anagram() & SKIP ALREADY GROUPED WORDS USING A `visited` VECTOR. [ O(N*N *K) | O(N)+O(N*K) ]
    // check for Anagrams
    pub fn is_anagram(s: &str, t: &str) -> bool {
        if s.len() != t.len() { return false; }
        let mut freq = [0; 26];
        for c in s.bytes() { freq[(c - b'a') as usize] += 1; }
        for c in t.bytes() { freq[(c - b'a') as usize] -= 1; }
        freq.iter().all(|&x| x == 0)
    }

    // grouping anagrams
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = Vec::new();

        let mut visited = vec![false; strs.len()];

        for i in 0..strs.len() {
            if visited[i] { continue; }
            let mut group = vec![strs[i].clone()];
            visited[i] = true;

            for j in (i + 1)..strs.len() {
                if !visited[j] && Self::is_anagram(&strs[i], &strs[j]) {
                    group.push(strs[j].clone());
                    visited[j] = true;
                }
            }res.push(group);
        }
        res
    }



/// OPTIMAL: SORTING EACH WORD'S CHARACTERS & GROUPS ANAGRAMS EFFICIENTLY BY USING A HashMap. [ O(N *K logK) | O(NK)+O(NK) ]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key = chars.iter().collect::<String>();

            map.entry(key).or_default().push(s);
        }
        map.into_values().collect()
    }

}


// MAIN
fn main() {

    let strs: Vec<String> = ["eat","tea","tan","ate","nat","bat"].iter().map(|s| s.to_string()).collect();
    let ans = Solution::group_anagrams(strs);

    println!("Grouped Anagrams: {:?}", ans);

}
