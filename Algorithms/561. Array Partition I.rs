impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        
        nums.chunks(2).map(|i| i[0]).sum()
    }
}
