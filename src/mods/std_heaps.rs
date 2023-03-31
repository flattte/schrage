#![allow(unused)]
use crate::mods::task::{Task, QInvariant, RInvariant};
use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;



pub fn schrage_heaps_bh(tasks: Vec<Task>) -> (Vec<Task>, u32) {
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
pub fn schrage_heaps_bh_cmax(tasks: Vec<Task>) -> u32 {
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

pub fn schrage_preemptive_heaps_bh_cmax(tasks: Vec<Task>) -> u32 {
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

fn preemtivity() {}

#[cfg(test)]
mod tests {
    use crate::{correct_order, tasks};
    use super::*;

    #[test]
    fn test_schrage_heaps() {
        let tasks = tasks!();
        let cmax = schrage_heaps_bh_cmax(tasks);
        assert_eq!(cmax, 53);
    }

    #[test]
    fn test_schrage_heaps_order() {
        let tasks = tasks!();
        let (order, cmax) = schrage_heaps_bh(tasks);
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
