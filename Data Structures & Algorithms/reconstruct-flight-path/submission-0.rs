use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse; 

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        // construct the graph
        let mut g: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new(); 
        for t in tickets.iter() { 
            g.entry(t[0].clone()).or_insert(BinaryHeap::new()).push(Reverse(t[1].clone())); 
        }

        let mut result = Vec::new(); 
        Self::dfs("JFK".to_string(), &mut g, &mut result); 
        result.reverse(); 
        result
    }

    fn dfs(src: String, g: &mut HashMap<String, BinaryHeap<Reverse<String>>>, result: &mut Vec<String>) { 
        while let Some(heap) = g.get_mut(&src) { 
            if let Some(Reverse(airport)) = heap.pop() {
                Self::dfs(airport, g, result); 
            } else { 
                break; 
            }
        } 

        result.push(src); 
    }

}
