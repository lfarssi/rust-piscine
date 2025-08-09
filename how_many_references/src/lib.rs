pub use std::rc::Rc;
use std::{
    alloc::{alloc, dealloc, Layout},
    ops::Deref,
};

pub struct Wrapper<T> {
    ptr: *mut T,
}
impl<T> Wrapper<T> {
    pub fn new(x: T) -> Self {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = alloc(layout) as *mut T;
            *ptr = x;
            return Self { ptr };
        }
    }
}
impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return unsafe { &*self.ptr };
    }
}
impl<T> Drop for Wrapper<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr as *mut u8, Layout::new::<T>());
        }
    }
}

pub struct Generous<T> {
    ptr: *mut T,
    count: *mut usize,
}
impl<T> Generous<T> {
    pub fn new(x: T) -> Self {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = alloc(layout) as *mut T;
            *ptr = x;
            let count = alloc(Layout::new::<usize>()) as *mut usize;
            *count = 1;
            return Self { ptr, count: count };
        }
    }
}
impl<T> Deref for Generous<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
impl<T> Clone for Generous<T> {
    fn clone(&self) -> Self {
        unsafe { *self.count += 1 };
        Self {
            ptr: self.ptr,
            count: self.count,
        }
    }
}
impl<T> Drop for Generous<T> {
    fn drop(&mut self) {
        unsafe {
            *self.count -= 1;
            println!("{}", *self.count);
            if *self.count == 0 {
                dealloc(self.ptr as *mut u8, Layout::new::<T>());
                dealloc(self.count as *mut u8, Layout::new::<usize>());
            }
        }
    }
}

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Self { ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|e| !Rc::ptr_eq(e, &element));
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::<String>::strong_count(ref_list)
}
