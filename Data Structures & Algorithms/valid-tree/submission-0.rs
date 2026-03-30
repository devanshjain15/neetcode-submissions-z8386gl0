use std::collections::HashSet; 

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
        for e in edges {
            g.entry(e[0]).or_insert(Vec::new()).push(e[1]);
            g.entry(e[1]).or_insert(Vec::new()).push(e[0]);
        }

        let mut visited = HashMap::new(); 
        if !Self::dfs(0, -1, &g, &mut visited) { 
            return false;
        } 

        visited.len() == n as usize
    }

    fn dfs(cur: i32, parent: i32, g: &HashMap<i32, Vec<i32>>, visited: &mut HashMap<i32, i32>) -> bool { 
        visited.insert(cur, parent); 

        if let Some(edges) = g.get(&cur) { 
            for e in edges.iter() { 
                if !visited.contains_key(e) { 
                    if !Self::dfs(*e, cur, g, visited) { 
                        return false; 
                    }
                } else if *e != parent { 
                    return false; 
                }
            }
        }
        
        true
    }
}
