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

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.len() == 0 {
            return None;
        }

        if self.heap.len() == 1 {
            return self.heap.pop();
        }

        let result = self.heap.swap_remove(0);
        self.sift_down(0);
        Some(result)
    }

    pub fn sift_down(&mut self, starting_node: usize) {
        let left_child = 2 * starting_node + 1;
        let right_child = 2 * starting_node + 2;
        let mut smallest = starting_node;

        if left_child < self.heap.len() && self.heap[left_child] < self.heap[smallest] {
            smallest = left_child;
        }

        if right_child < self.heap.len() && self.heap[right_child] < self.heap[smallest] {
            smallest = right_child;
        }

        if smallest != starting_node {
            self.heap.swap(smallest, starting_node);
            self.sift_down(smallest);
        }
    }

    pub fn sift_up(&mut self, starting_node: usize) {
        let mut current = starting_node;
        let mut parent = (current.saturating_sub(1)) / 2;
        while current > 0 && self.heap[current] < self.heap[parent] {
            self.heap.swap(current, parent);
            current = parent;
            parent = (current.saturating_sub(1)) / 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mods::task::Task;

    use super::*;

    #[test]
    fn test_heap() {
        let mut heap: HeapTree<Task> = HeapTree::new();
        heap.push(Task::new(1, 1, 1));
        heap.push(Task::new(2, 2, 2));
        heap.push(Task::new(3, 3, 3));
        heap.push(Task::new(4, 4, 4));

        assert_eq!(heap.pop(), Some(Task::new(1, 1, 1)));
        assert_eq!(heap.pop(), Some(Task::new(2, 2, 2)));
        assert_eq!(heap.pop(), Some(Task::new(3, 3, 3)));
        assert_eq!(heap.pop(), Some(Task::new(4, 4, 4)));
        assert_eq!(heap.pop(), None);
    }
}