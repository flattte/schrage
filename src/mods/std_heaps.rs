#![allow(unused)]
use crate::mods::task::Task;
use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;

#[derive(Eq, Debug)]
pub struct QInvariant(Task);

impl From<Task> for RInvariant {
    fn from(task: Task) -> Self {
        RInvariant(task)
    }
}
impl From<&Task> for RInvariant {
    fn from(task: &Task) -> Self {
        RInvariant(*task)
    }
}
impl PartialEq for RInvariant {
    fn eq(&self, other: &Self) -> bool {
        self.0.r == other.0.r
    }
}
impl PartialOrd for RInvariant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.r.cmp(&self.0.r))
    }
}
// since RInvariant has to be in descending order
//the order of comparison is (other) <=> (self) and not the other way around
impl Ord for RInvariant {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.r.cmp(&self.0.r)
    }
}

#[derive(Eq, Debug)]
pub struct RInvariant(Task);

impl From<Task> for QInvariant {
    fn from(task: Task) -> Self {
        QInvariant(task)
    }
}
impl From<&Task> for QInvariant {
    fn from(task: &Task) -> Self {
        QInvariant(*task)
    }
}
impl PartialEq for QInvariant {
    fn eq(&self, other: &Self) -> bool {
        self.0.q == other.0.q
    }
}
impl PartialOrd for QInvariant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.q.cmp(&other.0.q))
    }
}
impl Ord for QInvariant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.q.cmp(&other.0.q)
    }
}

#[derive(Debug)]
pub struct ShrageContextBH {
    pub available_tasks: BinaryHeap<QInvariant>,
    pub unavailable_tasks: BinaryHeap<RInvariant>,
}

impl ShrageContextBH {
    pub fn new() -> ShrageContextBH {
        ShrageContextBH {
            unavailable_tasks: BinaryHeap::new(),
            available_tasks: BinaryHeap::new(),
        }
    }

    pub fn from_vec(tasks: &Vec<Task>) -> ShrageContextBH {
        ShrageContextBH {
            unavailable_tasks: BinaryHeap::from_iter(tasks.iter().map(|t| t.into())),
            available_tasks: BinaryHeap::new(),
        }
    }
}

pub fn shrage_heaps_bh(tasks: Vec<Task>) -> (Vec<Task>, u32) {
    let mut ctx = ShrageContextBH::from_vec(&tasks);
    let mut t = 0;
    let mut cmax = 0;
    let mut order = Vec::new();

    // heaps make code cleaner and more imperative
    while !ctx.available_tasks.is_empty() || !ctx.unavailable_tasks.is_empty() {
        // unwrap is safe beacause the while loop condition
        while !ctx.unavailable_tasks.is_empty() && ctx.unavailable_tasks.peek().unwrap().0.r <= t {
            let task = ctx.unavailable_tasks.pop().unwrap().0;
            ctx.available_tasks.push(task.into());
        }
        if ctx.available_tasks.is_empty() {
            // unwrap is safe, if the available_tasks is empty
            // then the unavailable_tasks is not empty
            // because of the while loop condition
            t = ctx.unavailable_tasks.peek().unwrap().0.r;
            continue;
        }

        let task = ctx.available_tasks.pop().unwrap().0;
        t += task.p;
        cmax = max(cmax, t + task.q);
        order.push(task);
    }

    (order, cmax)
}

// just cmax
pub fn shrage_heaps_bh_cmax(tasks: Vec<Task>) -> u32 {
    let mut ctx = ShrageContextBH::from_vec(&tasks);
    let mut t = 0;
    let mut cmax = 0;

    while !ctx.available_tasks.is_empty() || !ctx.unavailable_tasks.is_empty() {
        while !ctx.unavailable_tasks.is_empty() && ctx.unavailable_tasks.peek().unwrap().0.r <= t {
            let task = ctx.unavailable_tasks.pop().unwrap().0;
            ctx.available_tasks.push(task.into());
        }

        if ctx.available_tasks.is_empty() {
            t = ctx.unavailable_tasks.peek().unwrap().0.r;
            continue;
        }

        let task = ctx.available_tasks.pop().unwrap().0;
        t += task.p;
        cmax = max(cmax, t + task.q);
    }

    cmax
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use crate::{correct_order, tasks};

    #[allow(unused)]
    use super::*;

    #[test]
    fn test_shrage_heaps() {
        let tasks = tasks!();
        let cmax = shrage_heaps_bh_cmax(tasks);
        assert_eq!(cmax, 53);
    }

    #[test]
    fn test_shrage_heaps_order() {
        let tasks = tasks!();
        let (order, cmax) = shrage_heaps_bh(tasks);
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
