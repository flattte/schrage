#![allow(unused)]
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct HeapTree<T> {
    pub heap: Vec<T>,
}


impl<T: Ord> HeapTree<T> {
    pub fn new() -> HeapTree<T> {
        HeapTree { heap: Vec::new() }
    }

    pub fn push(&mut self, elem: T) {
        self.heap.push(elem);
        self.sift_up(self.heap.len() - 1);
    }

    pub fn pop(&mut self, elem: T) -> Option<T> {
        if self.heap.is_empty() {
            return None
        }
        
        let elem = self.heap.ta
    }

    pub fn sift_down(&mut self, starting_node: usize) {

    }

    pub fn sift_up(&mut self, starting_node: usize) {

    }
}