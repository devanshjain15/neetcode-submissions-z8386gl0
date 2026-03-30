impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len(); 
        let mut cached = HashMap::new(); 
        Self::helper(0, &cost, n, &mut cached).min(Self::helper(1, &cost, n, &mut cached))
    }

    fn helper(cur: usize, cost: &Vec<i32>, n: usize, cached: &mut HashMap<usize, i32>) -> i32 { 
        if cur >= n {
            return 0;
        }

        if cached.contains_key(&cur) { 
            return *cached.get(&cur).unwrap(); 
        }

        let c = cost[cur] + Self::helper(cur + 1, cost, n, cached).min(Self::helper(cur + 2, cost, n, cached));
        cached.insert(cur, c); 

        c
    }
}
