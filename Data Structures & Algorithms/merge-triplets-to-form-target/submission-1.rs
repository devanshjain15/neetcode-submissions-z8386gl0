use std::collections::HashSet; 

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let (x, y, z) = (target[0], target[1], target[2]); 

        let mut found_x = false; 
        let mut found_y = false; 
        let mut found_z = false; 

        for t in triplets.iter() { 
            if t[0] > x || t[1] > y || t[2] > z { 
                continue; 
            }

            if t[0] == x { found_x = true; }
            if t[1] == y { found_y = true; }
            if t[2] == z { found_z = true; }
        }

        found_x && found_y && found_z 
    }
}
