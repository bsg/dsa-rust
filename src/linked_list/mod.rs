use std::mem;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            item,
            next: mem::take(&mut self.head),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        mem::take(&mut self.head).map(|node| {
            self.head = node.next;
            node.item
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.item
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.item
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut link = mem::take(&mut self.head);
        while let Some(mut node) = link {
            link = mem::take(&mut node.next);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn push_pop() {
        let mut list = LinkedList::new();
        (0..10).for_each(|n| list.push(n));
        (0..10).rev().for_each(|n| assert_eq!(list.pop(), Some(n)));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        list.peek_mut().map(|item| *item = 3);
        assert_eq!(list.peek(), Some(&3));
    }
}
