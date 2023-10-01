#![allow(dead_code)]

use std::{
    alloc,
    fmt::Debug,
    sync::{atomic, atomic::AtomicUsize, Condvar, Mutex, RwLock},
};

#[derive(Debug, PartialEq)]
pub enum BoundedQueueError {
    Full,
}

pub struct BoundedQueue<T> {
    front: AtomicUsize,
    rear: AtomicUsize,

    write_event: (Mutex<()>, Condvar),
    read_event: (Mutex<()>, Condvar),

    buffer: RwLock<Box<[T]>>,
}

unsafe impl<T> Send for BoundedQueue<T> {}
unsafe impl<T> Sync for BoundedQueue<T> {}

impl<T: Copy> BoundedQueue<T> {
    pub fn new(size: usize) -> BoundedQueue<T> {
        let buffer_layout = alloc::Layout::array::<T>(size + 1).unwrap();
        let boxed_buffer: Box<[T]>;
        unsafe {
            let ptr = alloc::alloc(buffer_layout) as *mut T;
            let buffer = core::slice::from_raw_parts_mut(ptr, size + 1);
            boxed_buffer = Box::from_raw(buffer);
        }
        BoundedQueue {
            front: AtomicUsize::new(0),
            rear: AtomicUsize::new(0),
            read_event: (Mutex::new(()), Condvar::new()),
            write_event: (Mutex::new(()), Condvar::new()),
            buffer: RwLock::new(boxed_buffer),
        }
    }

    pub fn enqueue(&self, item: T) -> Result<(), BoundedQueueError> {
        let mut buffer = self.buffer.write().unwrap();
        let front = self.front.load(atomic::Ordering::Relaxed);
        let rear = self.rear.load(atomic::Ordering::Relaxed);

        if (front + 1) % buffer.len() == rear {
            Err(BoundedQueueError::Full)
        } else {
            buffer[front] = item;
            self.front
                .store((front + 1) % buffer.len(), atomic::Ordering::Relaxed);
            self.notify_write();
            Ok(())
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        let buffer = self.buffer.read().unwrap();
        let front = self.front.load(atomic::Ordering::Relaxed);
        let rear = self.rear.load(atomic::Ordering::Relaxed);

        if rear == front {
            None
        } else {
            let item = buffer[rear].clone();
            self.rear
                .store((rear + 1) % buffer.len(), atomic::Ordering::Relaxed);
            self.notify_read();
            Some(item)
        }
    }

    fn notify_read(&self) {
        self.read_event.1.notify_one();
    }

    fn notify_write(&self) {
        self.write_event.1.notify_one();
    }

    fn wait_read(&self) {
        let read = self.read_event.0.lock().unwrap();
        let _guard = self.read_event.1.wait(read).unwrap();
    }

    fn wait_write(&self) {
        let written = self.write_event.0.lock().unwrap();
        let _guard = self.write_event.1.wait(written).unwrap();
    }

    pub fn enqueue_blocking(&self, item: T) {
        match self.enqueue(item) {
            Ok(_) => (),
            Err(BoundedQueueError::Full) => {
                self.wait_read();
                self.enqueue_blocking(item);
            }
        }
    }

    pub fn dequeue_blocking(&self) -> T {
        match self.dequeue() {
            Some(item) => item,
            None => {
                self.wait_write();
                self.dequeue_blocking()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, RwLock},
        thread,
    };

    use crate::queue::BoundedQueueError;

    use super::BoundedQueue;

    #[test]
    fn single_item() {
        let queue: BoundedQueue<u32> = BoundedQueue::new(1);
        queue.enqueue(1).unwrap();
        assert_eq!(queue.dequeue(), Some(1));
        queue.enqueue(2).unwrap();
        assert_eq!(queue.dequeue(), Some(2));
    }

    #[test]
    fn multiple_items() {
        let queue: BoundedQueue<u32> = BoundedQueue::new(5);
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        queue.enqueue(4).unwrap();
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
    }

    #[test]
    fn full() {
        let queue: BoundedQueue<u32> = BoundedQueue::new(1);
        queue.enqueue(1).unwrap();
        assert_eq!(queue.enqueue(2).expect_err(""), BoundedQueueError::Full);
    }

    #[test]
    fn spsc() {
        let queue: Arc<RwLock<BoundedQueue<u32>>> = Arc::new(RwLock::new(BoundedQueue::new(1)));
        let sender = queue.clone();
        let receiver = queue.clone();

        thread::spawn(move || {
            let tx = sender.read().unwrap();
            tx.enqueue_blocking(1);
            tx.enqueue_blocking(2);
            tx.enqueue_blocking(3);
        });

        let rx = receiver.read().unwrap();
        assert_eq!(1, rx.dequeue_blocking());
        assert_eq!(2, rx.dequeue_blocking());
        assert_eq!(3, rx.dequeue_blocking());
    }

    #[test]
    fn mpsc() {
        let queue: Arc<RwLock<BoundedQueue<u32>>> = Arc::new(RwLock::new(BoundedQueue::new(1)));
        let sender1 = queue.clone();
        let sender2 = queue.clone();
        let receiver = queue.clone();

        thread::spawn(move || {
            let tx = sender1.read().unwrap();
            tx.enqueue_blocking(1);
            tx.enqueue_blocking(1);
            tx.enqueue_blocking(1);
        });

        thread::spawn(move || {
            let tx = sender2.read().unwrap();
            tx.enqueue_blocking(1);
            tx.enqueue_blocking(1);
            tx.enqueue_blocking(1);
        });

        let rx = receiver.read().unwrap();
        assert_eq!(1, rx.dequeue_blocking());
        assert_eq!(1, rx.dequeue_blocking());
        assert_eq!(1, rx.dequeue_blocking());
        assert_eq!(1, rx.dequeue_blocking());
        assert_eq!(1, rx.dequeue_blocking());
        assert_eq!(1, rx.dequeue_blocking());
    }
}
