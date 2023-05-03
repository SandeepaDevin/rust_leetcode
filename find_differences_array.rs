use std::collections::HashSet;
impl Solution {

    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {

        fn filter(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
            let set: HashSet<_> = v2.iter().collect(); 
            let mut result: Vec<_> = v1.iter().filter(|x| !set.contains(x)).map(|&x| x).collect();
            result.sort_unstable(); 
            result.dedup(); 
            result
        }

        let num3: Vec<i32> = nums2.clone();
        let num4: Vec<i32> = nums1.clone();
        let sub1 = vec![filter(nums1,nums2),filter(num3,num4)];
        sub1
    }
}
