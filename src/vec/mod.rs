#![allow(dead_code)]

use std::{
    alloc,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
};

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.cap {
            if self.len == 0 {
                self.grow(1);
            } else {
                self.grow(self.len * 2);
            }
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), item);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len);

        if self.len == self.cap {
            self.grow(self.len * 2);
        }

        unsafe {
            ptr::copy(
                self.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.as_mut_ptr().add(index), item);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);

        unsafe {
            let item = ptr::read(self.as_ptr().add(index));
            ptr::copy(
                self.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index - 1,
            );
            self.len -= 1;
            item
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn reserve(&mut self, len: usize) {
        self.grow(len);
    }

    // TODO make this safer
    fn grow(&mut self, new_size: usize) {
        let old_layout = alloc::Layout::array::<T>(self.cap).unwrap();
        let new_layout = alloc::Layout::array::<T>(new_size).unwrap();

        unsafe {
            let ptr = if self.cap == 0 {
                alloc::alloc(new_layout) as *mut T
            } else {
                alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size())
                    as *mut T
            };
            self.ptr = NonNull::<T>::new(ptr).unwrap();
        }

        self.cap = new_size;
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = alloc::Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr().cast(), layout) };
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T: Clone> From<&[T]> for Vec<T> {
    fn from(slice: &[T]) -> Vec<T> {
        let mut vec = Self::new();
        vec.grow(slice.len());
        for item in slice {
            vec.push((*item).clone());
        }
        vec
    }
}

impl<T: Clone> From<&mut [T]> for Vec<T> {
    fn from(slice: &mut [T]) -> Vec<T> {
        let mut vec = Self::new();
        vec.grow(slice.len());
        for item in slice {
            vec.push((*item).clone());
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::Vec;

    #[test]
    fn push_pop_one() {
        let mut vec = Vec::new();
        vec.push(1);
        assert_eq!(vec.pop(), Some(1));
    }

    #[test]
    fn push_pop_four() {
        let mut vec = Vec::new();
        vec.push(1);
        assert_eq!(vec.capacity(), 1);
        vec.push(2);
        assert_eq!(vec.capacity(), 2);
        vec.push(3);
        assert_eq!(vec.capacity(), 4);
        vec.push(4);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.pop(), Some(4));
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
    }

    #[test]
    fn deref() {
        let mut vec = Vec::from([1, 2, 3].as_slice());
        assert_eq!(vec[0], 1);
        vec[0] = 2;
        assert_eq!(vec[0], 2);
        assert_eq!(vec[1..=2], [2, 3]);
    }

    #[test]
    fn insert() {
        let mut vec = Vec::from([1, 2, 3].as_slice());
        vec.insert(1, 5);
        assert_eq!(vec.len, 4);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 5);
        assert_eq!(vec[2], 2);
        assert_eq!(vec[3], 3);
    }

    #[test]
    fn remove() {
        let mut vec = Vec::from([1, 2, 3, 4, 5].as_slice());
        assert_eq!(vec.remove(2), 3);
        assert_eq!(vec.len, 4);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 4);
        assert_eq!(vec[3], 5);
    }
}
