use std::collections::{HashMap, VecDeque}; 

impl Solution {
    pub fn foreign_dictionary(words: Vec<String>) -> String {
        // let mut g: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut g = vec![vec![false; 26]; 26];  
        let mut indegree = [0; 26];
        let mut exists = [false; 26]; 

        let mut n = 0; 
        for word in words.iter() { 
            for &c in word.as_bytes() { 
                let idx = (c - b'a') as usize;
                if !exists[idx] {
                    exists[idx] = true;
                    n += 1;
                }
            }
        }

        for i in 0..(words.len() - 1) { 
            let w1 = words[i].as_bytes(); 
            let w2 = words[i + 1].as_bytes(); 

            if w1.len() > w2.len() && w1.starts_with(w2) { 
                return String::new(); 
            }

            for j in 0..w1.len().min(w2.len()) { 
                if w1[j] != w2[j] { 
                    let u = (w1[j] - b'a') as usize; 
                    let v = (w2[j] - b'a') as usize; 
                    if !g[u][v] { // don't understand use of this 
                        g[u][v] = true; 
                        indegree[v] += 1; 
                    }
                    break; 
                }
            }
        }

        let mut result = String::new(); 
        let mut queue = VecDeque::new(); 
        let mut count = 0; 

        for i in 0..26 { 
            if indegree[i] == 0 && exists[i] { 
                queue.push_back(i as u8); 
            }
        }

        while let Some(u) = queue.pop_front() { 
            result.push((u + b'a') as char); 
            count += 1; 
            for v in 0..26 { 
                if g[u as usize][v] { 
                    indegree[v] -= 1; 
                    if indegree[v] == 0 { 
                        queue.push_back(v as u8); 
                    }
                }
            }
        }

        if count != n { 
            return String::new(); 
        }

        result
    }
}
