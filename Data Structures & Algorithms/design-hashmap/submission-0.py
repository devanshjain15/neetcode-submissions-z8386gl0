class Node: 
    def __init__(self, key: int = 0, value: int = 0): 
        self.key = key
        self.value = value
        self.next = None

class MyHashMap:
    def __init__(self):
        self.map = [Node() for _ in range(10**4)]

    def put(self, key: int, value: int) -> None:
        index = key % 10000
        temp = self.map[index] 
        while temp.next: 
            temp = temp.next
            if temp and temp.key == key: 
                temp.value = value
                return

        temp.next = Node(key, value)
        

    def get(self, key: int) -> int:
        index = key % 10000
        temp = self.map[index] 
        while temp.next: 
            if temp.next.key == key: 
                return temp.next.value
            temp = temp.next
        return -1        

    def remove(self, key: int) -> None:
        index = key % 10000
        temp = self.map[index] 
        while temp.next: 
            if temp.next.key == key: 
                temp.next = temp.next.next
                return 




# Your MyHashMap object will be instantiated and called as such:
# obj = MyHashMap()
# obj.put(key,value)
# param_2 = obj.get(key)
# obj.remove(key)