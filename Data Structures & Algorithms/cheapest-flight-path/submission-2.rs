impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut prices = vec![i32::MAX; n as usize]; 

        prices[src as usize] = 0; 

        for i in 0..=k {
            let mut temp = prices.clone(); 

            for flight in flights.iter() { 
                let s = flight[0] as usize; 
                if prices[s] == i32::MAX { 
                    continue; 
                }
                let d = flight[1] as usize; 
                let p = flight[2]; 

                if prices[s] + p < temp[d] { 
                    temp[d] = prices[s] + p; 
                } 
            }

            prices = temp; 
        }
    

        if prices[dst as usize] == i32::MAX { 
            -1
        } else { 
            prices[dst as usize]
        }
    }
}
