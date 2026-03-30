use std::collections::{HashMap, HashSet, VecDeque}; 

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_set: HashSet<String> = word_list.into_iter().collect(); 
        if !word_set.contains(&end_word) { 
            return 0; 
        }

        let mut q = VecDeque::new(); 
        q.push_back((begin_word.clone(), 1)); 
        while let Some((word, level)) = q.pop_front() { 
            if word == end_word {
                return level; 
            }

            let mut chars: Vec<char> = word.chars().collect(); 
            for i in 0..chars.len() { 
                let original = chars[i]; 
                for c in 'a'..='z' { 
                    if c == original { 
                        continue; 
                    }
                    chars[i] = c; 
                    let w: String = chars.iter().collect(); 
                    if word_set.contains(&w) { 
                        q.push_back((w.clone(), level + 1)); 
                        word_set.remove(&w); 
                    }
                }

                chars[i] = original; 
            }
        }

        0
    }
}