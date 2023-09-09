use std::{alloc, ptr::NonNull};

type NodeRef<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    value: T,
    parent: NodeRef<T>,
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
        let mut node_ref = &mut self.root;
        let mut parent = None;

        unsafe {
            while let Some(node) = node_ref {
                parent = Some(*node);
                node_ref = if value > node.as_ref().value {
                    &mut node.as_mut().right
                } else {
                    &mut node.as_mut().left
                }
            }

            let mut new_node = Self::alloc_node();
            new_node.as_mut().value = value;
            new_node.as_mut().parent = parent;
            new_node.as_mut().left = None;
            new_node.as_mut().right = None;

            let _ = node_ref.insert(new_node);
        }
    }

    pub fn contains(&self, value: T) -> bool {
        self.locate(value).is_some()
    }

    pub fn remove(&mut self, value: T) -> bool {
        let node_ref = self.locate_mut_ptr(value);
        unsafe {
            if (*node_ref).is_none() {
                return false;
            };

            match (
                (*node_ref).unwrap().as_ref().left,
                (*node_ref).unwrap().as_ref().right,
            ) {
                (None, None) => {
                    (*node_ref).take();
                }
                (None, Some(mut child)) | (Some(mut child), None) => {
                    child.as_mut().parent.replace((*node_ref).unwrap());
                    (*node_ref).replace(child);
                }
                (Some(left), Some(right)) => {
                    let mut successor = self.successor((*node_ref).unwrap().as_ref()).unwrap(); // unwrap won't fail
                    if successor != right {
                        successor.as_mut().right = successor.as_ref().parent;
                    }
                    successor.as_mut().left.replace(left);
                    (*node_ref).replace(successor);
                }
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

    fn locate(&self, value: T) -> NodeRef<T> {
        let mut node_ref = &self.root;

        unsafe {
            while let Some(node) = *node_ref {
                if value == node.as_ref().value {
                    return *node_ref;
                } else if value > node.as_ref().value {
                    node_ref = &node.as_ref().right;
                } else {
                    node_ref = &node.as_ref().left;
                }
            }
        }

        None
    }

    fn locate_mut_ptr(&mut self, value: T) -> *mut NodeRef<T> {
        let mut node_ref = &mut self.root;

        unsafe {
            while let Some(mut node) = *node_ref {
                if value == node.as_ref().value {
                    return &mut *node_ref as *mut NodeRef<T>;
                } else if value > node.as_ref().value {
                    node_ref = &mut node.as_mut().right;
                } else {
                    node_ref = &mut node.as_mut().left;
                }
            }
        }

        &mut None
    }

    // TODO see if making this take &mut Node<T> makes miri happy
    fn successor(&mut self, node: &Node<T>) -> NodeRef<T> {
        let mut node_ref = node;

        unsafe {
            if let Some(right) = node.right {
                node_ref = right.as_ref();
                while let Some(left) = node_ref.left {
                    node_ref = left.as_ref();
                }
                Some(node_ref.into())
            } else {
                todo!()
            }
        }
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

    #[test]
    fn remove_node_with_one_child() {
        let mut tree: BSTree<i32> = BSTree::new();
        let items = [5, 1, 7, 2];
        items.iter().for_each(|item| tree.insert(*item));
        tree.remove(1);
        assert!(!tree.contains(1));
    }

    #[test]
    fn remove_node_with_child_successor() {
        let mut tree: BSTree<i32> = BSTree::new();
        let items = [7, 6, 8];
        items.iter().for_each(|item| tree.insert(*item));
        tree.remove(7);
        assert!(!tree.contains(7));
        assert!(tree.contains(6));
        assert!(tree.contains(8));
    }

    #[test]
    fn remove_node_with_non_child_successor() {
        let mut tree: BSTree<i32> = BSTree::new();
        let items = [3, 1, 5, 4, 6];
        items.iter().for_each(|item| tree.insert(*item));
        tree.remove(3);
        assert!(!tree.contains(3));
        assert!(tree.contains(1));
        assert!(tree.contains(5));
        assert!(tree.contains(4));
        assert!(tree.contains(6));
    }
}
