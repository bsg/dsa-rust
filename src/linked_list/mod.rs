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
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
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

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> std::iter::Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.item
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> std::iter::Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.item
        })
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> std::iter::Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
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

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        (0..10).for_each(|n| list.push(n));
        (0..10)
            .into_iter()
            .rev()
            .zip(list.iter())
            .for_each(|(x, y)| assert_eq!(x, *y));
        list.iter_mut().for_each(|item| *item = 0);
        assert_eq!(list.iter().sum::<u32>(), 0);
    }
}
