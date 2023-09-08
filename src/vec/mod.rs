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
            self.grow();
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
        todo!()
    }

    pub fn remove(&mut self, index: usize) {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    fn grow(&mut self) {
        if self.len == 0 {
            let layout = alloc::Layout::array::<T>(1).unwrap();
            unsafe { self.ptr = NonNull::<T>::new(alloc::alloc(layout) as *mut T).unwrap() };
            self.cap = 1;
        } else {
            let old_layout = alloc::Layout::array::<T>(self.cap).unwrap();
            let new_layout = alloc::Layout::array::<T>(self.cap * 2).unwrap();
            unsafe {
                self.ptr = NonNull::<T>::new(alloc::realloc(
                    self.ptr.as_ptr() as *mut u8,
                    old_layout,
                    new_layout.size(),
                ) as *mut T)
                .unwrap()
            }
            self.cap *= 2;
        }
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
        for item in slice {
            vec.push((*item).clone());
        }
        vec
    }
}

impl<T: Clone> From<&mut [T]> for Vec<T> {
    fn from(slice: &mut [T]) -> Vec<T> {
        let mut vec = Self::new();
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
}
