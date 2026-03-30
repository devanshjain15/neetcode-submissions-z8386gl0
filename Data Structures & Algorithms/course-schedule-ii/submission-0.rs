use std::collections::{HashMap, VecDeque}; 

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize; 
        let mut g = HashMap::new(); 
        for p in prerequisites.iter() { 
            g.entry(p[1]).or_insert(Vec::new()).push(p[0]); 
        }

        let mut in_degree = vec![0; n]; 
        for i in 0..n { 
            if let Some(p_vec) = g.get(&(i as i32)) { 
                for p in p_vec.iter() {
                    in_degree[*p as usize] += 1; 
                }
            }
        }

        let mut queue = VecDeque::new(); 
        for i in 0..n { 
            if in_degree[i] == 0 { 
                queue.push_back(i as i32); 
            }
        }

        let mut order = vec![0; n]; 
        let mut count = 0; 
        while let Some(v) = queue.pop_front() { 
            order[count] = v; 
            count += 1; 
            if let Some(edges) = g.get(&v) { 
                for e in edges.iter() {
                    in_degree[*e as usize] -= 1;  
                    if in_degree[*e as usize] == 0 { 
                        queue.push_back(*e); 
                    }
                }
            }
        }

        if count == n { 
            order
        } else { 
            vec![] 
        }
    }
}