impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut compared = HashMap::new();
        for index2 in 0..nums.len(){
            match compared.get(&(target - nums[index2])){
                Some(index1) => return vec![*index1,index2 as i32,],
                None => compared.insert(nums[index2], index2 as i32)
                };
            }
        vec![0,0]
        }
    }