impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for index in 0..nums.len(){
            let diff = target - nums[index];
            for index2 in index+1..(nums.len()){
                if &nums[index2]== &diff{
                    return vec![index as i32,index2 as i32]
                }
            }
        }
        vec![0,0]
    }
}