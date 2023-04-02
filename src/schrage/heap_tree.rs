#![allow(unused)]
use std::{collections::BinaryHeap, fmt::Debug};


#[derive(Debug, Default)]
pub struct HeapTree<T> {
    pub heap: Vec<T>,
}

impl<T: Ord> From<Vec<T>> for HeapTree<T> {
    fn from(vec: Vec<T>) -> HeapTree<T> {
        let mut heap = HeapTree { heap: vec };
        heap.heapify_vec();
        heap
    }
}

impl<T: Ord> FromIterator<T> for HeapTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> HeapTree<T> {
        HeapTree::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T: Ord> HeapTree<T> {
    pub fn new() -> HeapTree<T> {
        HeapTree { heap: Vec::new() }
    }

    pub fn push(&mut self, elem: T) {
        self.heap.push(elem);
        self.sift_up(self.heap.len() - 1);
    }

    pub fn sift_up(&mut self, starting_node: usize) {
        let mut current = starting_node;
        let mut parent = (current.saturating_sub(1)) / 2;
        while current > 0 && self.heap[current] > self.heap[parent] {
            self.heap.swap(current, parent);
            current = parent;
            parent = (current.saturating_sub(1)) / 2;
        }
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

    pub fn sift_down(&mut self, mut index: usize) {
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut smallest = index;
            if left < self.heap.len() && self.heap[left] > self.heap[smallest] {
                smallest = left;
            }
            if right < self.heap.len() && self.heap[right] > self.heap[smallest] {
                smallest = right;
            }
            if smallest != index {
                self.heap.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }

    // assumeses that self.heap is random distributed vec
    fn heapify_vec(&mut self) {
        for i in (0..=self.heap.len() / 2).rev() {
            self.sift_down(i)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.get(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schrage::task::{QInvariant, RInvariant, Task};
    use crate::{correct_order, tasks};

    #[test]
    fn test_custom_heap() {
        let mut heap: HeapTree<Task> = HeapTree::new();
        heap.push(Task::new(4, 4, 4));
        heap.push(Task::new(2, 2, 2));
        heap.push(Task::new(1, 1, 1));
        heap.push(Task::new(3, 3, 3));
        assert_eq!(heap.pop(), Some(Task::new(4, 4, 4)));
        assert_eq!(heap.pop(), Some(Task::new(3, 3, 3)));
        assert_eq!(heap.pop(), Some(Task::new(2, 2, 2)));
        assert_eq!(heap.pop(), Some(Task::new(1, 1, 1)));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_comparisons_rinvariant() {
        let mut heap: HeapTree<RInvariant> = tasks!().iter().map(|t| t.into()).collect();
        assert_eq!(heap.pop().unwrap(), Task::new(0, 6, 17).into());
    }

    #[test]
    fn test_comparisons_qinvariant() {
        let mut heap: HeapTree<QInvariant> = tasks!().iter().map(|t| t.into()).collect();
        assert_eq!(heap.pop().unwrap().0, Task::new(13, 6, 26).into());
    }
}
