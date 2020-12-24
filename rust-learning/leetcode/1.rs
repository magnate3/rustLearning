pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                None => { map.insert(num, index); },
                Some(sub_index) => { return vec![*sub_index as i32, index as i32] },
            }
			println!("{:#?}",map);
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}

fn main() {
	let t=Solution::two_sum(vec![1,2,3,11, 15,5,7,12 ], 9);
	println!("{:#?}",t);
}