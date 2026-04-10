impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_gas = 0; 
        let mut total_cost = 0; 
        let mut tank = 0; 
        let mut start = 0; 

        for i in 0..gas.len() { 
            total_gas += gas[i]; 
            total_cost += cost[i]; 
            tank += gas[i]; 
            tank -= cost[i]; 

            if tank < 0 { 
                start = i + 1; 
                tank = 0; 
            }
        }

        if total_gas < total_cost { 
            return -1; 
        }

        start as i32
    }
}