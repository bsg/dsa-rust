use std::mem;

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            item,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let result;
        match mem::replace(&mut self.head, None) {
            Some(e) => {
                result = Some(e.item);
                self.head = e.next;
            }
            None => result = None,
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn push_pop() {
        let mut list = List::new();
        (0..10).for_each(|n| list.push(n));
        (0..10).rev().for_each(|n| assert_eq!(list.pop(), Some(n)));
    }
}
