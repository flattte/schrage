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
        if self.heap.is_empty() {
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

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.get(0)
    }

    // assumeses that self.heap is random distributed vec
    fn heapify_vec(&mut self) {
        for i in (0..=self.heap.len() / 2).rev() {
            self.sift_down(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapify_vec() {
        let mut heap = HeapTree::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(heap.heap, vec![9, 8, 7, 4, 5, 6, 3, 2, 1]);
        heap.heapify_vec();
        assert_eq!(heap.heap, vec![9, 8, 7, 4, 5, 6, 3, 2, 1]);
    }

    #[test]
    fn test_push() {
        let mut heap = HeapTree::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);
        heap.push(5);
        heap.push(6);
        heap.push(7);
        heap.push(8);
        heap.push(9);
        assert_eq!(heap.pop(), Some(9));
    }

    #[test]
    fn test_pop() {
        let mut heap = HeapTree::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_sift_down() {
        let mut heap = HeapTree::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        heap.sift_down(0);
        assert_eq!(heap.heap, vec![9, 8, 7, 4, 5, 6, 3, 2, 1]);
    }
}
