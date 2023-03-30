use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;
use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub struct Task {
    pub r: u32, // ready time
    pub p: u32, // working time
    pub q: u32, // cooldown time
}

impl Task {
    pub fn new(r: u32, p: u32, q: u32) -> Task {
        Task { r, p, q }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.p, self.q)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.p == other.p && self.q == other.q
    }
}

#[derive(Debug)]
pub struct QInvariant(Task);
#[derive(Debug)]
pub struct RInvariant(Task);

impl From<&Task> for RInvariant {
    fn from(task: &Task) -> Self {
        RInvariant(*task)
    }
}

impl From<&Task> for QInvariant {
    fn from(task: &Task) -> Self {
        QInvariant(*task)
    }
}

impl From<Task> for RInvariant {
    fn from(task: Task) -> Self {
        RInvariant(task)
    }
}

impl From<Task> for QInvariant {
    fn from(task: Task) -> Self {
        QInvariant(task)
    }
}

impl PartialEq for QInvariant {
    fn eq(&self, other: &Self) -> bool {
        self.0.q == other.0.q
    }
}

impl Eq for QInvariant {}

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

impl PartialEq for RInvariant {
    fn eq(&self, other: &Self) -> bool {
        self.0.r == other.0.r
    }
}

impl Eq for RInvariant {}

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

#[derive(Debug)]
pub struct ShrageContext {
    pub available_tasks: BinaryHeap<QInvariant>,
    pub unavailable_tasks: BinaryHeap<RInvariant>,
}

impl ShrageContext {
    pub fn new() -> ShrageContext {
        ShrageContext {
            unavailable_tasks: BinaryHeap::new(),
            available_tasks: BinaryHeap::new(),
        }
    }

    pub fn from_vec(tasks: &Vec<Task>) -> ShrageContext {
        ShrageContext {
            unavailable_tasks: BinaryHeap::from_iter(tasks.iter().map(|t| t.into())),
            available_tasks: BinaryHeap::new(),
        }
    }
}

pub fn shrage_heaps(tasks: Vec<Task>) -> (Vec<Task>, u32) {
    let mut ctx = ShrageContext::from_vec(&tasks);
    let mut t = 0;
    let mut cmax = 0;
    let mut order = Vec::new();

    while !ctx.available_tasks.is_empty() || !ctx.unavailable_tasks.is_empty() {
        // unwrap is safe beacause the while loop condition
        while !ctx.unavailable_tasks.is_empty() && ctx.unavailable_tasks.peek().unwrap().0.r <= t {
            let task = ctx.unavailable_tasks.pop().unwrap().0;
            ctx.available_tasks.push(task.into());
        }

        // unwrap is safe, if the available_tasks is empty
        // then the unavailable_tasks is not empty
        // because of the while loop condition
        if ctx.available_tasks.is_empty() {
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

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_shrage_heaps() {
        let tasks = vec![
            Task::new(10, 5, 7),
            Task::new(13, 6, 26),
            Task::new(11, 7, 24),
            Task::new(20, 4, 21),
            Task::new(30, 3, 8),
            Task::new(0, 6, 17),
            Task::new(30, 2, 0),
        ];

        let (_, cmax) = shrage_heaps(tasks);
        assert_eq!(cmax, 53);
    }

    #[test]
    fn test_comparisons_rinvariant() {
        let t1 = Task::new(10, 5, 7);
        let t2 = Task::new(13, 6, 26);
        let t3 = Task::new(11, 7, 24);
        let t4 = Task::new(20, 4, 21);
        let t5 = Task::new(30, 3, 8);
        let t6 = Task::new(0, 6, 17);
        let t7 = Task::new(30, 2, 0);

        let mut heap: BinaryHeap<RInvariant> = BinaryHeap::new();
        heap.push(t1.into());
        heap.push(t2.into());
        heap.push(t3.into());
        heap.push(t4.into());
        heap.push(t5.into());
        heap.push(t6.into());
        heap.push(t7.into());

        assert_eq!(heap.pop().unwrap().0, t6);
    }

    #[test]
    fn test_comparisons_qinvariant() {
        let t1 = Task::new(10, 5, 7);
        let t2 = Task::new(13, 6, 26);
        let t3 = Task::new(11, 7, 24);
        let t4 = Task::new(20, 4, 21);
        let t5 = Task::new(30, 3, 8);
        let t6 = Task::new(0, 6, 17);
        let t7 = Task::new(30, 2, 0);

        let mut heap: BinaryHeap<QInvariant> = BinaryHeap::new();
        heap.push(t1.into());
        heap.push(t2.into());
        heap.push(t3.into());
        heap.push(t4.into());
        heap.push(t5.into());
        heap.push(t6.into());
        heap.push(t7.into());

        assert_eq!(heap.pop().unwrap().0, t2);
    }
}
