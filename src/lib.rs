#[derive(Debug)]
struct MyLinkedList {
    len: i32,
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
        if index >= self.len {
            return -1;
        }

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
        if self.len == 0 {
            self.add_at_head(val);
            return;
        }

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
        if index == 0 {
            self.add_at_head(val);
            return;
        } else if index == self.len {
            self.add_at_tail(val);
            return;
        }

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
        if index >= self.len {
            return;
        }

        let mut current_link = self.head.as_mut();
        if index == 0 {
            self.head = current_link.unwrap().next.take();
        } else {
            for _ in 0..index - 1 {
                current_link = current_link.unwrap().next.as_mut();
            }
            let next_node = current_link
                .as_mut() // why does it need as_mut()?
                .unwrap()
                .next
                .as_mut()
                .unwrap()
                .next
                .take();
            current_link.as_mut().unwrap().next = next_node;
        }
        self.len -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(1);
        obj.add_at_tail(3);
        obj.add_at_index(1, 2);
        assert_eq!(2, obj.get(1));
        obj.delete_at_index(1);
        assert_eq!(3, obj.get(1));
    }

    #[test]
    fn test2() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(1);
        obj.delete_at_index(0);
    }

    #[test]
    fn test3() {
        let mut obj = MyLinkedList::new();
        obj.add_at_index(0, 10);
        obj.add_at_index(0, 20);
        obj.add_at_index(1, 30);
        assert_eq!(20, obj.get(0));
    }

    #[test]
    fn test4() {
        let mut obj = MyLinkedList::new();
        obj.add_at_tail(1);
        assert_eq!(1, obj.get(0));
    }

    #[test]
    fn test5() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(2);
        obj.delete_at_index(1);
        obj.add_at_head(2);
        obj.add_at_head(7);
        obj.add_at_head(3);
        obj.add_at_head(2);
        obj.add_at_head(5);
        obj.add_at_tail(5);
        assert_eq!(2, obj.get(5));
        obj.delete_at_index(6);
        obj.delete_at_index(4);
    }

    #[test]
    fn test6() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(4);
        assert_eq!(-1, obj.get(1));
        obj.add_at_head(1);
        obj.add_at_head(5);
        obj.delete_at_index(3);
        obj.add_at_head(7);
        assert_eq!(4, obj.get(3));
        assert_eq!(4, obj.get(3));
        assert_eq!(4, obj.get(3));
        obj.add_at_head(1);
        obj.delete_at_index(4);
    }
}
