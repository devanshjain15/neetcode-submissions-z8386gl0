class Node: 
    def __init__(self,key, value): 
        self.key, self.val = key, value; 
        self.next = self.prev = None; 

class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity; 
        self.map = {}; 
        self.lru, self.mru = Node(0, 0), Node(0, 0); 
        self.lru.next = self.mru; 
        self.mru.prev = self.lru; 

    def remove(self, node): 
        prev, next = node.prev, node.next; 
        prev.next, next.prev = next, prev; 

    def insert(self, node): 
        prev = self.mru.prev; 
        prev.next = node; 
        node.next = self.mru;
        node.prev = prev;  
        self.mru.prev = node; 

    def get(self, key: int) -> int:
        if key not in self.map: 
            return -1; 

        node = self.map[key];
        self.remove(node); 
        self.insert(node); 

        return node.val; 
        

    def put(self, key: int, value: int) -> None:
        node = None
        if key in self.map: 
            node = self.map[key]; 
            self.map[key].val = value; 
            self.remove(node);  
        else: 
            node = Node(key, value);
            self.map[key] = node; 
         
        self.insert(node); 

        if len(self.map) > self.capacity: 
            lru = self.lru.next; 
            self.remove(lru); 
            del self.map[lru.key];

        

        
