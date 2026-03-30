use std::collections::HashMap; 

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // buy => decision = true;
        // sell => decision = false; 
        let n = prices.len() as i32; 
        Self::helper(0, true, n, &prices, &mut HashMap::new())
    }

    fn helper(i: i32, decision: bool, n: i32, prices: &Vec<i32>, cache: &mut HashMap<(i32, bool), i32>) -> i32 { 
        if i >= n { 
            return 0; 
        }

        if cache.contains_key(&(i, decision)) { 
            return *cache.get(&(i, decision)).unwrap(); 
        }

        let mut profit = 0; 
        let cooldown = Self::helper(i + 1, decision, n, prices, cache); 
        if decision { 
            profit = cooldown.max(Self::helper(i + 1, !decision, n, prices, cache) - prices[i as usize]); 
        } else { 
            profit = cooldown.max(Self::helper(i + 2, !decision, n, prices, cache) + prices[i as usize]); 
        }

        cache.insert((i, decision), profit); 
        return profit; 
    }
}
