impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable(); 

        let mut result = Vec::new(); 
        let mut combination = Vec::new(); 

        Self::helper(0, &candidates, 0, &mut combination, &mut result, target); 
        result
    }

    fn helper(start: usize, candidates: &Vec<i32>, sum: i32, combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, target: i32) { 
        if sum == target { 
            result.push(combination.clone()); 
            return; 
        }

        if sum > target { 
            return; 
        }


        for i in start..candidates.len() { 
            if i > start && candidates[i] == candidates[i - 1] { 
                continue; 
            }
            let e = candidates[i]; 
            combination.push(e); 
            Self::helper(i + 1, candidates, sum + e, combination, result, target); 
            combination.pop(); 
        }
    }
}
