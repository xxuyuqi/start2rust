struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:HashMap<i32,usize>=HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let k=target-num;          
            if let Some(ii) = map.get(&k){
                if *ii != i{
                    return vec![*ii as i32,i as i32];
                }                
            }
             map.insert(*num, i);
        }        
        vec![]
    }
}

fn main() {
    let nums = vec![3,2,4];
    let res = Solution::two_sum(nums, 6);
    println!("{:?}", res);
}