use std::collections::{HashMap, VecDeque}; 
use std::cell::RefCell;
use std::rc::Rc;

/*
// Definition for a Node.
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}
*/

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut result = None; 

        let mut map: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new(); 
        let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new(); 

        if let Some(n) = node.clone() {
            let start = Rc::new(RefCell::new(Node::new(n.borrow().val))); 
            map.insert(n.borrow().val, start.clone()); 
            result = Some(start); 
            queue.push_back(n); 
        }

        while let Some(node) = queue.pop_front() { 
            let cur_node = map.get(&node.borrow().val).unwrap().clone();

            for neighbor in node.borrow().neighbors.iter() {
                let neighbor = neighbor.clone();  
                if !map.contains_key(&neighbor.borrow().val) {
                    let node = Rc::new(RefCell::new(Node::new(neighbor.borrow().val))); 
                    map.insert(neighbor.borrow().val, node.clone()); 
                    queue.push_back(neighbor.clone()); 
                }
                cur_node.borrow_mut().neighbors.push(map.get(&neighbor.borrow().val).unwrap().clone()); 
            } 
            
        }

        result
    }
}
