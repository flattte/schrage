use crate::task::{QInvariant, RInvariant, Task};
use heap::HeapTree;
use std::cmp::max;

pub fn schrage_custom_heaps(tasks: Vec<Task>) -> (Vec<Task>, u32) {
    let mut available_tasks: HeapTree<QInvariant> = HeapTree::new();
    let mut unavailable_tasks: HeapTree<RInvariant> = tasks.iter().map(|t| t.into()).collect();
    let mut t = 0;
    let mut cmax = 0;
    let mut order = Vec::new();

    // heaps make code cleaner and more imperative
    while !available_tasks.is_empty() || !unavailable_tasks.is_empty() {
        // unwrap is safe beacause the while loop condition
        while !unavailable_tasks.is_empty() && unavailable_tasks.peek().unwrap().0.r <= t {
            let task: Task = unavailable_tasks.pop().unwrap().into();
            available_tasks.push(task.into());
        }
        if available_tasks.is_empty() {
            // unwrap is safe, if the available_tasks is empty
            // then the unavailable_tasks is not empty
            // because of the while loop condition
            t = unavailable_tasks.peek().unwrap().0.r;
            continue;
        }

        let task = available_tasks.pop().unwrap().0;
        t += task.p;
        cmax = max(cmax, t + task.q);
        order.push(task);
    }

    (order, cmax)
}

// just cmax
pub fn schrage_custom_heaps_cmax(tasks: Vec<Task>) -> u32 {
    let mut available_tasks: HeapTree<QInvariant> = HeapTree::new();
    let mut unavailable_tasks: HeapTree<RInvariant> = tasks.iter().map(|t| t.into()).collect();
    let mut t = 0;
    let mut cmax = 0;

    while !available_tasks.is_empty() || !unavailable_tasks.is_empty() {
        while !unavailable_tasks.is_empty() && unavailable_tasks.peek().unwrap().0.r <= t {
            let task: Task = unavailable_tasks.pop().unwrap().into();
            available_tasks.push(task.into());
        }

        if available_tasks.is_empty() {
            t = unavailable_tasks.peek().unwrap().0.r;
            continue;
        }

        let task = available_tasks.pop().unwrap().0;
        t += task.p;
        cmax = max(cmax, t + task.q);
    }

    cmax
}

pub fn schrage_preemptive_custom_heaps_cmax(tasks: Vec<Task>) -> u32 {
    let mut available_tasks: HeapTree<QInvariant> = HeapTree::new();
    let mut unavailable_tasks: HeapTree<RInvariant> = tasks.iter().map(|t| t.into()).collect();
    let mut t = 0;
    let mut cmax = 0;

    let mut current_task: Option<Task> = None;

    while !available_tasks.is_empty() || !unavailable_tasks.is_empty() {
        while !unavailable_tasks.is_empty() && unavailable_tasks.peek().unwrap().0.r <= t {
            let task: Task = unavailable_tasks.pop().unwrap().into();
            available_tasks.push(task.into());
            if let Some(mut current) = current_task {
                if task.q > current.q {
                    current.p = t - task.r;
                    t = task.r;
                    if current.p > 0 {
                        available_tasks.push(current.into())
                    }
                }
            }
        }

        if available_tasks.is_empty() {
            t = unavailable_tasks.peek().unwrap().0.r;
            continue;
        }

        let task_to_do = available_tasks.pop().unwrap().0;
        t += task_to_do.p;
        cmax = max(cmax, t + task_to_do.q);
        current_task = Some(task_to_do);
    }

    cmax
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tasks {
        () => {
            vec![
                Task::new(30, 3, 8),
                Task::new(20, 4, 21),
                Task::new(10, 5, 7),
                Task::new(11, 7, 24),
                Task::new(30, 2, 0),
                Task::new(13, 6, 26),
                Task::new(0, 6, 17),
            ]
        };
    }

    macro_rules! correct_order {
        () => {
            vec![
                Task::new(0, 6, 17),
                Task::new(10, 5, 7),
                Task::new(13, 6, 26),
                Task::new(11, 7, 24),
                Task::new(20, 4, 21),
                Task::new(30, 3, 8),
                Task::new(30, 2, 0),
            ]
        };
    }

    #[test]
    fn test_schrage_heaps() {
        let tasks = tasks!();
        let cmax = schrage_custom_heaps_cmax(tasks);
        assert_eq!(cmax, 53);
    }

    #[test]
    fn test_schrage_heaps_order() {
        let tasks = tasks!();
        let (order, cmax) = schrage_custom_heaps(tasks);
        assert_eq!(cmax, 53);
        assert_eq!(order, correct_order!());
    }

    #[test]
    fn test_comparisons_rinvariant() {
        let mut heap: HeapTree<RInvariant> = tasks!().iter().map(|t| t.into()).collect();
        let aa = heap.pop().unwrap();
        assert_eq!(aa, Task::new(0, 6, 17).into());
        //assert_eq!(heap.pop().unwrap(), Task::new(0, 6, 17).into());
    }

    #[test]
    fn test_comparisons_qinvariant() {
        let mut heap: HeapTree<QInvariant> = tasks!().iter().map(|t| t.into()).collect();
        assert_eq!(heap.pop().unwrap().0, Task::new(13, 6, 26).into());
    }

    #[test]
    fn test_std_heap() {
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
}
