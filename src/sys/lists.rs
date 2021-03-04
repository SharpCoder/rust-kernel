/*
    Author: Josh Cole
    
    This is a rust implementation of a system level linked list. It supports
    standard operations including pop, and get.
*/
#![allow(dead_code)]
use crate::sys::mem::{ kalloc, free };

#[derive(Copy, Clone)]
struct Node<T : Copy> {
    item: T,
    next: Option<*mut Node<T>>,
}

pub struct Stack<T : Copy> {
    head: Option<*mut Node<T>>,
    size: usize,
}

impl <T : Copy> Stack<T> {
    pub fn new() -> Self {
        return Stack { head: None, size: 0 };
    }

    pub fn push(&mut self, item: T) {
        let ptr = kalloc();
        unsafe {
            (*ptr) = Node {
                item: item,
                next: self.head,
            };
        }

        self.head = Some(ptr);
        self.size = self.size + 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            None => {
                return None;
            },
            Some(node) => {
                // Copy the reference
                let indirect = node.clone();
                let node_item = unsafe { *indirect };
                
                // Free the actual node.
                free(node);

                let result = node_item.item;
                self.head = node_item.next;
                self.size = self.size - 1;
                return Some(result);
            },
        };  
    }
    
    pub fn size(&self) -> usize {
        return self.size;
    }
}

#[cfg(test)]
mod test { 

    use super::*;

    #[test]
    fn stack() {
        let mut list = Stack::new();
        list.push(32);
        list.push(64);
        list.push(128);
        list.push(256);

        assert_eq!(list.size(), 4);
        assert_eq!(list.pop(), Some(256));
        assert_eq!(list.size(), 3);
        assert_eq!(list.pop(), Some(128));
        assert_eq!(list.size(), 2);
        assert_eq!(list.pop(), Some(64));
        assert_eq!(list.size(), 1);
        assert_eq!(list.pop(), Some(32));
        assert_eq!(list.size(), 0);
        assert_eq!(list.pop(), None);

    }

}