use std::collections::{HashMap, VecDeque}; 

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new(); 
        for p in prerequisites.iter() { 
            g.entry(p[1]).or_insert(Vec::new()).push(p[0]); 
        }

        let n = num_courses as usize; 
        let mut in_degree = vec![0; n]; 
        for i in 0..n { 
            if let Some(edges) = g.get(&(i as i32)) { 
                for &e in edges.iter() { 
                    in_degree[e as usize] += 1; 
                }
            }
        }

        let mut queue = VecDeque::new(); 
        for i in 0..n { 
            if in_degree[i] == 0 { 
                queue.push_back(i as i32); 
            }
        }

        let mut count = 0; 
        while let Some(v) = queue.pop_front() { 
            count += 1;
            if let Some(edges) = g.get(&v) { 
                for &e in edges.iter() { 
                    let idx = e as usize; 
                    in_degree[idx] -= 1; 
                    if in_degree[idx] == 0 { 
                        queue.push_back(idx as i32); 
                    }
                }
            }
        }

        count == n
    }
}
