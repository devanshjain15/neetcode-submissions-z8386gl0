use std::collections::HashSet; 

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total_sum: i32 = nums.iter().sum(); 
        if total_sum % 2 != 0 { 
            return false; 
        }

        let target = total_sum / 2; 
        let mut dp = HashSet::new(); 
        dp.insert(0); 

        for &num in nums.iter() { 
            let mut temp = dp.clone(); 
            for &sum in dp.iter() {
                if sum + num == target { 
                    return true; 
                }
                temp.insert(sum + num); 
                // temp.insert(num); 
                temp.insert(sum); 
            }

            dp = temp; 
        }

        for &sum in dp.iter() { 
            if sum == target { 
                return true; 
            }
        }

        false
    }
}
