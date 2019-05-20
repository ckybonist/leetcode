// Given an array of integers, return indices of the two numbers such that they add up to a specific target.
//
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
//
// Example:
//
//
// Given nums = [2, 7, 11, 15], target = 9,
//
// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].
//
//
//  
//


use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sum = 0;
        let mut result: Vec<i32> = vec![-1; 2];
        
        // HashMap<num: i32, index: i32>
        let mut map = Solution::memorize(&nums);
        
        for (i, elem) in nums.iter().enumerate() {
            let diff = target - elem;
            if let Some(j) = map.get(&diff) {
                if *j != i {
                    result[0] = i as i32;
                    result[1] = *j as i32;
                    break;
                }
            }
        }
        
        result
    }
    
    fn memorize(v: &Vec<i32>) -> HashMap<i32, usize> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (index, elem) in v.iter().enumerate() {
            map.insert(*elem, index);
        }

        map
    }
}
