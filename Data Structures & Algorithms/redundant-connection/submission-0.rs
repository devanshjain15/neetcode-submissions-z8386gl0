struct UnionFind {
    parent: Vec<usize>, 
    rank: Vec<usize>
}

impl UnionFind { 
    pub fn new(n: usize) -> Self{ 
        let mut parent = Vec::with_capacity(n + 1); 
        let rank = vec![0; n + 1]; 
        for i in 0..=n { 
            parent.push(i);
        }

        Self { 
            parent, 
            rank
        }
    }

    fn find(&mut self, x: usize) -> usize { 
        if self.parent[x] != x {
            let root = self.find(self.parent[x]); 
            self.parent[x] = root; 
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool { 
        let root_x = self.find(x); 
        let root_y = self.find(y); 

        if root_x == root_y { 
            return false; 
        } 

        if self.rank[root_x] < self.rank[root_y] { 
            self.parent[root_x] = root_y; 
        } else if self.rank[root_x] > self.rank[root_y] { 
            self.parent[root_y] = root_x; 
        } else { 
            self.parent[root_y] = root_x; 
            self.rank[root_x] += 1; 
        }

        true
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len(); 
        let mut uf = UnionFind::new(n); 

        for e in edges { 
            let u = e[0] as usize; 
            let v = e[1] as usize; 
            if !uf.union(u, v) { 
                return vec![u as i32, v as i32]; 
            }
        }

        vec![]
    }
}
