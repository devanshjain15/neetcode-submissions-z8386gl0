use std::collections::{HashMap, HashSet}; 

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut list: HashMap<i32, Vec<i32>> = HashMap::new(); 

        for prerequisite in prerequisites.iter() { 
            list.entry(prerequisite[0]).or_insert(Vec::new()).push(prerequisite[1]); 
        }

        let mut state: Vec<i32> = vec![0; num_courses as usize]; 
        for i in 0..(num_courses as usize) { 
            if !Self::dfs(i as i32, &list, &mut state) { 
                return false; 
            } 
        }

        true
    }

    fn dfs(c: i32, list: &HashMap<i32, Vec<i32>>, state: &mut Vec<i32>) -> bool {
        let idx = c as usize; 

        if state[idx] == 1 { 
            return false; 
        }

        if state[idx] == 2 { 
            return true; 
        }

        state[idx] = 1; 
        if let Some(p_vec) = list.get(&c) { 
            for p in p_vec.iter() { 
                if !Self::dfs(*p, &list, state) { 
                    return false; 
                } 
            }
        }

        state[idx] = 2; 
        true
    }
}
