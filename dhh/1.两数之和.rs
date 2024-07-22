/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: std::collections::HashMap<i32, i32>  = std::collections::HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j, i as i32];
            }
            map.insert(*num, i as i32);
        }
        vec![]
    }
}
