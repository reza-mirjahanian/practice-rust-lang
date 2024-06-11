impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash = HashMap::new();
        
        for i in 0..nums.len() {
            if !hash.contains_key(&(target - nums[i])) {
                hash.insert(nums[i], i);
            } else {
                return vec![hash[&(target - nums[i])] as i32, i as i32];
            }
        }
        return vec![];
    }
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            match hash.get(&(target - num)) {
                Some(&index) => return vec![index as i32, i as i32],
                None => {
                    hash.insert(num, i);
                }
            }
        }

        vec![]
    }
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = hash.get(&complement) {
                return vec![j as i32, i as i32];
            }
            hash.insert(num, i);
        }
        vec![]
    }
}
