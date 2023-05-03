use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.drain(..).collect();
        nums.extend(set);
        nums.sort_unstable();
        nums.len() as i32    
    }
}
