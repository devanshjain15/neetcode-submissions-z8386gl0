use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse; 

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize; 
        let mut g = HashMap::new(); 
        for t in times.iter() { 
            g.entry(t[0]).or_insert(Vec::new()).push((t[1], t[2])); 
        }


        let mut d = vec![i32::MAX; n + 1]; 
        d[k as usize] = 0; 
        let mut pq: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new(); 
        pq.push(Reverse((k, 0)));

        while let Some(Reverse((node, time))) = pq.pop() {
            if time > d[node as usize] { 
                continue; 
            }

            if let Some(neighbors) = g.get(&node) { 
                for &(next, dt) in neighbors.iter() { 
                    let w = time + dt; 
                    if w < d[next as usize] { 
                        d[next as usize] = w; 
                        pq.push(Reverse((next, w)));
                    }
                }
            }
        }

        let mut ans = i32::MIN; 
        for i in 1..=n { 
            if d[i] == i32::MAX { 
                return -1; 
            }

            ans = ans.max(d[i]); 
        }

        ans
    }
}