use std::collections::BinaryHeap; 
use std::cmp::Reverse; 

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len(); 
        let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new(); 
        
        heap.push((Reverse(0), 0));
        let mut visited = vec![false; n]; 
        let mut total_cost = 0; 
        let mut edge_count = 0; 

        while edge_count < n { 
            if let Some((Reverse(cost), u)) = heap.pop() { 
                if visited[u] { 
                    continue;
                }

                visited[u] = true;  
                total_cost += cost; 
                edge_count += 1; 

                for i in 0..n { 
                    if !visited[i] { 
                        let d = (points[i][0] - points[u][0]).abs() + (points[i][1] - points[u][1]).abs(); 
                        heap.push((Reverse(d), i)); 
                    }
                }
            } else { 
                break; 
            }
        }

        total_cost
    }
}
