use std::{alloc, fmt::Debug};

pub struct BoundedQueue<T> {
    front: usize,
    rear: usize,
    buffer: Box<[T]>,
}

#[derive(Debug, PartialEq)]
pub enum BoundedQueueError {
    Full,
}

impl<T> BoundedQueue<T> {
    pub fn new(size: usize) -> BoundedQueue<T> {
        let buffer_layout = alloc::Layout::array::<T>(size + 1).unwrap();
        let boxed_buffer: Box<[T]>;
        unsafe {
            let ptr = alloc::alloc(buffer_layout) as *mut T;
            let buffer = core::slice::from_raw_parts_mut(ptr, size + 1);
            boxed_buffer = Box::from_raw(buffer);
        }
        BoundedQueue {
            front: 0,
            rear: 0,
            buffer: boxed_buffer,
        }
    }

    pub fn enqueue(&mut self, item: T) -> Result<(), BoundedQueueError> {
        if (self.front + 1) % self.buffer.len() == self.rear {
            Err(BoundedQueueError::Full)
        } else {
            self.buffer[self.front] = item;
            self.front = (self.front + 1) % self.buffer.len();
            Ok(())
        }
    }

    pub fn dequeue(&mut self) -> Option<&T> {
        if self.rear == self.front {
            None
        } else {
            let item = &self.buffer[self.rear];
            self.rear = (self.rear + 1) % self.buffer.len();
            Some(item)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::BoundedQueueError;

    use super::BoundedQueue;

    #[test]
    fn single_item() {
        let mut queue: BoundedQueue<u32> = BoundedQueue::new(1);
        queue.enqueue(1).unwrap();
        assert_eq!(queue.dequeue(), Some(&1));
        queue.enqueue(2).unwrap();
        assert_eq!(queue.dequeue(), Some(&2));
    }

    #[test]
    fn multiple_items() {
        let mut queue: BoundedQueue<u32> = BoundedQueue::new(5);
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        queue.enqueue(4).unwrap();
        assert_eq!(queue.dequeue(), Some(&1));
        assert_eq!(queue.dequeue(), Some(&2));
        assert_eq!(queue.dequeue(), Some(&3));
        assert_eq!(queue.dequeue(), Some(&4));
    }

    #[test]
    fn full() {
        let mut queue: BoundedQueue<u32> = BoundedQueue::new(1);
        queue.enqueue(1).unwrap();
        assert_eq!(queue.enqueue(2).expect_err(""), BoundedQueueError::Full);
    }
}
