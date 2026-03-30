use std::collections::{HashMap, HashSet}; 

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut visited = HashSet::new(); 
        let mut g = HashMap::new(); 
        
        for e in edges.iter() { 
            g.entry(e[0]).or_insert(Vec::new()).push(e[1]); 
            g.entry(e[1]).or_insert(Vec::new()).push(e[0]); 
        }

        let mut count = 0; 
        for i in 0..n {
            let i = i as i32; 
            if !visited.contains(&i) { 
                Self::dfs(i, &mut visited, &g); 
                count += 1; 
            }  
        }
        count
    }

    fn dfs(cur: i32, visited: &mut HashSet<i32>, g: &HashMap<i32, Vec<i32>>) { 
        visited.insert(cur); 

        if let Some(edges) = g.get(&cur) { 
            for e in edges.iter() { 
                if !visited.contains(e) { 
                    Self::dfs(*e, visited, g);
                }
            }
        }
    }
}
