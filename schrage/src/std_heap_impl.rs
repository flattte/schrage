use crate::task::{QInvariant, RInvariant, Task};
use std::cmp::max;
use std::collections::BinaryHeap;

pub fn schrage_heaps_std(tasks: Vec<Task>) -> (Vec<Task>, u32) {
    let mut available_tasks: BinaryHeap<QInvariant> = BinaryHeap::new();
    let mut unavailable_tasks: BinaryHeap<RInvariant> = tasks.iter().map(|t| t.into()).collect();
    let mut t = 0;
    let mut cmax = 0;
    let mut order = Vec::new();

    // heaps make code cleaner and more imperative
    while !available_tasks.is_empty() || !unavailable_tasks.is_empty() {
        // unwrap is safe beacause the while loop condition
        while !unavailable_tasks.is_empty() && unavailable_tasks.peek().unwrap().0.r <= t {
            let task = unavailable_tasks.pop().unwrap().0;
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
pub fn schrage_heaps_std_cmax(tasks: Vec<Task>) -> u32 {
    let mut available_tasks: BinaryHeap<QInvariant> = BinaryHeap::new();
    let mut unavailable_tasks: BinaryHeap<RInvariant> = tasks.iter().map(|t| t.into()).collect();
    let mut t = 0;
    let mut cmax = 0;

    while !available_tasks.is_empty() || !unavailable_tasks.is_empty() {
        while !unavailable_tasks.is_empty() && unavailable_tasks.peek().unwrap().0.r <= t {
            let task = unavailable_tasks.pop().unwrap().0;
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

pub fn schrage_preemptive_heaps_std_cmax(tasks: Vec<Task>) -> u32 {
    let mut available_tasks: BinaryHeap<QInvariant> = BinaryHeap::new();
    let mut unavailable_tasks: BinaryHeap<RInvariant> = tasks.iter().map(|t| t.into()).collect();
    let mut t = 0;
    let mut cmax = 0;

    let mut current_task: Option<Task> = None;

    while !available_tasks.is_empty() || !unavailable_tasks.is_empty() {
        while !unavailable_tasks.is_empty() && unavailable_tasks.peek().unwrap().0.r <= t {
            let task: Task = unavailable_tasks.pop().unwrap().into();
            available_tasks.push(task.into());
            if let Some(ref mut current) = current_task {
                if task.q > current.q {
                    current.p = t - task.r;
                    t = task.r;
                    if current.p > 0 {
                        available_tasks.push(current.to_owned().into())
                    }
                }
            }
        }

        if available_tasks.is_empty() {
            t = unavailable_tasks.peek().unwrap().0.r;
            current_task = None;
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
        let cmax = schrage_heaps_std_cmax(tasks);
        assert_eq!(cmax, 53);
    }

    #[test]
    fn test_schrage_heaps_order() {
        let tasks = tasks!();
        let (order, cmax) = schrage_heaps_std(tasks);
        assert_eq!(cmax, 53);
        assert_eq!(order, correct_order!());
    }

    #[test]
    fn test_comparisons_rinvariant() {
        let mut heap: BinaryHeap<RInvariant> = tasks!().iter().map(|t| t.into()).collect();
        assert_eq!(heap.pop().unwrap(), Task::new(0, 6, 17).into());
    }

    #[test]
    fn test_comparisons_qinvariant() {
        let mut heap: BinaryHeap<QInvariant> = tasks!().iter().map(|t| t.into()).collect();
        assert_eq!(heap.pop().unwrap().0, Task::new(13, 6, 26).into());
    }
}
