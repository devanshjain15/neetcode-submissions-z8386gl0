use std::collections::{HashMap, HashSet}; 

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let chars: Vec<char> = s.chars().collect(); 
        let n = chars.len(); 
        let mut last = HashMap::new(); 
        for (i, &c) in chars.iter().enumerate() { 
            last.insert(c, i); 
        }
        let mut ans = vec![]; 
        let mut start = 0; 
        let mut end = 0; 
        for i in 0..n { 
            let c = chars[i]; 
            if last[&c] > end { 
                end = last[&c]; 
            }

            if i == end { 
                ans.push((end - start + 1) as i32); 
                start = i + 1; 
            }
        }


        ans

        // instead of frequency map use last index based
        // let mut freq = HashMap::new(); 
        // for &c in chars.iter() { 
        //     *freq.entry(c).or_insert(0) += 1; 
        // }

        // let mut set = HashSet::new(); 
        // let mut ans = vec![]; 
        // let mut l = 0; 
        // for i in 0..chars.len() { 
        //     let c = chars[i]; 
        //     l += 1; 
        //     if !set.contains(&c) { 
        //         set.insert(c); 
        //     } else { 
        //         *set.entry(c) -= 1; 

        //         if *set.entry(c) == 0 { 
        //             set.remove(c); 
        //         }
        //     }

        //     if set.len() == 0 { 
        //         ans.push(l); 
        //         l = 0; 
        //     }
        // }

        // ans
    }
}
