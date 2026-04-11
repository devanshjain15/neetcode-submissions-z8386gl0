use std::collections::BTreeMap; 

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len() as i32; 
        if n % group_size != 0 { 
            return false; 
        }

        let mut t_map = BTreeMap::new(); 

        for &h in hand.iter() { 
            *t_map.entry(h).or_insert(0) += 1; 
        }

        while let Some((&start, _)) = t_map.first_key_value() { 
            for i in 0..group_size { 
                let next = start + i; 
                if let Some(count) = t_map.get_mut(&next) {
                    *count -= 1; 
                    if *count == 0 { 
                        t_map.remove(&next); 
                    }
                } else { 
                    return false; 
                }
            }
        }


        true
    }
}