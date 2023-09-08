use std::{alloc, ptr::NonNull, mem};

type NodeRef<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

pub struct BSTree<T> {
    root: NodeRef<T>,
}

impl<T: Eq + Ord> BSTree<T> {
    pub fn new() -> Self {
        BSTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let mut nodeRef = &mut self.root;

        unsafe {
            while let Some(node) = nodeRef {
                nodeRef = if value > node.as_ref().value {
                    &mut node.as_mut().right
                } else {
                    &mut node.as_mut().left
                }
            }

            let mut new_node = Self::alloc_node();
            new_node.as_mut().value = value;
            new_node.as_mut().left = None;
            new_node.as_mut().right = None;

            nodeRef.insert(new_node);
        }
    }

    pub fn contains(&self, value: T) -> bool {
        self.locate(value).1.is_some()
    }

    pub fn remove(&mut self, value: T) -> bool {
        let (mut parent, mut nodeRef) = self.locate(value);
        if nodeRef.is_none() {
            return false;
        };

        unsafe {
            match (
                nodeRef.unwrap().as_ref().left,
                nodeRef.unwrap().as_ref().right,
            ) {
                (None, None) => {
                    nodeRef.take(); // TODO
                },
                (None, Some(_)) => todo!(),
                (Some(_), None) => todo!(),
                (Some(_), Some(_)) => todo!(),
            }
        }

        true
    }

    fn alloc_node() -> NonNull<Node<T>> {
        let layout = alloc::Layout::new::<Node<T>>();
        unsafe {
            let ptr = alloc::alloc(layout) as *mut Node<T>;
            NonNull::new(ptr).unwrap()
        }
    }

    fn locate(&self, value: T) -> (NodeRef<T>, NodeRef<T>) {
        let mut nodeRef = &self.root;
        let mut parent = None;

        unsafe {
            while let Some(node) = *nodeRef {
                if value == node.as_ref().value {
                    return (parent, *nodeRef);
                } else if value > node.as_ref().value {
                    parent = Some(node);
                    nodeRef = &node.as_ref().right;
                } else {
                    parent = Some(node);
                    nodeRef = &node.as_ref().left;
                }
            }
        }

        (None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut tree: BSTree<i32> = BSTree::new();
        assert!(!tree.contains(1));
    }

    #[test]
    fn insert() {
        let mut tree: BSTree<i32> = BSTree::new();
        let items = [5, 3, 1, 6, 4];
        items.iter().for_each(|item| tree.insert(*item));
        items.iter().for_each(|item| assert!(tree.contains(*item)));
    }

    #[test]
    fn remove_leaf() {
        let mut tree: BSTree<i32> = BSTree::new();
        let items = [5, 3, 1, 6, 4];
        items.iter().for_each(|item| tree.insert(*item));
        tree.remove(4);
        assert!(!tree.contains(4));
    }
}
