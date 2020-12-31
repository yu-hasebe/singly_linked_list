#[derive(Debug)]
struct MyLinkedList {
    len: usize,
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self { len: 0, head: None }
    }

    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    fn get(&self, index: i32) -> i32 {
        let mut current_link = self.head.as_ref();
        for _ in 0..index {
            current_link = current_link.unwrap().next.as_ref();
        }
        current_link.unwrap().val
    }

    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    fn add_at_head(&mut self, val: i32) {
        let next = self.head.take();
        let new_node = Node { val, next };
        self.head = Some(Box::new(new_node));
        self.len += 1;
    }

    /** Append a node of value val to the last element of the linked list. */
    fn add_at_tail(&mut self, val: i32) {
        let mut current_link = self.head.as_mut();
        for _ in 0..self.len - 1 {
            current_link = current_link.unwrap().next.as_mut();
        }
        let next = None;
        let new_node = Node { val, next };
        current_link.as_mut().unwrap().next = Some(Box::new(new_node));
        self.len += 1;
    }

    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut current_link = self.head.as_mut();
        for _ in 0..index - 1 {
            current_link = current_link.unwrap().next.as_mut();
        }
        let next = current_link.as_mut().unwrap().next.take();
        let new_node = Node { val, next };
        current_link.as_mut().unwrap().next = Some(Box::new(new_node));
        self.len += 1;
    }

    /** Delete the index-th node in the linked list, if the index is valid. */
    fn delete_at_index(&mut self, index: i32) {
        let mut current_link = self.head.as_mut();
        for _ in 0..index - 1 {
            current_link = current_link.unwrap().next.as_mut();
        }
        let next_node = current_link
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .take();
        current_link.as_mut().unwrap().next = next_node;
        self.len -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(1);
        obj.add_at_tail(3);
        obj.add_at_index(1, 2);
        assert_eq!(2, obj.get(1));
        obj.delete_at_index(1);
        assert_eq!(3, obj.get(1));
    }
}
