impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut result = 0; 

        for i in 0..32 { 
            if (1 << i) & n != 0 { 
                result += 1; 
            }
        }


        result
    }
}
