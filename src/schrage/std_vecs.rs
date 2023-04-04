#![allow(unused)]
use crate::schrage::task::Task;
use std::cmp::max;

pub fn schrage_vecs_sort_q_cmax(mut tasks: Vec<Task>) -> u32 {
    let mut t = 0;
    let mut cmax = 0;
    tasks.sort_by(|a, b| a.q.cmp(&b.q));

    // additional scans though the vec make the code moref functional
    while !tasks.is_empty() {
        if let Some((idx, task)) = tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.r <= t)
            .last()
        {
            t += task.p;
            cmax = max(cmax, t + task.q);
            tasks.remove(idx);
        } else {
            t = tasks.iter().min_by(|a, b| a.r.cmp(&b.r)).unwrap().r;
        }
    }

    cmax
}

pub fn schrage_vecs_sort_q(mut tasks: Vec<Task>) -> (Vec<Task>, u32) {
    let mut t = 0;
    let mut cmax = 0;
    let mut order = Vec::new();
    tasks.sort_by(|a, b| a.q.cmp(&b.q));

    while !tasks.is_empty() {
        if let Some((idx, task)) = tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.r <= t)
            .last()
        {
            t += task.p;
            cmax = max(cmax, t + task.q);
            order.push(tasks.remove(idx));
        } else {
            t = tasks.iter().min_by(|a, b| a.r.cmp(&b.r)).unwrap().r;
        }
    }

    (order, cmax)
}

pub fn schrage_vecs_sort_r_cmax(mut tasks: Vec<Task>) -> u32 {
    let mut t = 0;
    let mut cmax = 0;
    tasks.sort_by(|a, b| a.r.cmp(&b.r));

    // additional scans though the vec make the code moref functional
    while !tasks.is_empty() {
        if let Some((idx, task)) = tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.r <= t)
            .max_by(|(_, a), (_, b)| a.q.cmp(&b.q))
        {
            t += task.p;
            cmax = max(cmax, t + task.q);
            tasks.remove(idx);
        } else {
            t = tasks.iter().min_by(|a, b| a.r.cmp(&b.r)).unwrap().r;
        }
    }

    cmax
}

pub fn schrage_vecs_sort_r(mut tasks: Vec<Task>) -> (Vec<Task>, u32) {
    let mut t = 0;
    let mut cmax = 0;
    let mut order = Vec::new();
    tasks.sort_by(|a, b| a.r.cmp(&b.r));

    // additional scans though the vec are necessary
    // the code is more functional
    while !tasks.is_empty() {
        if let Some((idx, task)) = tasks
            .iter()
            .enumerate()
            .take_while(|(_, task)| task.r <= t)
            .max_by(|(_, a), (_, b)| a.q.cmp(&b.q))
        {
            t += task.p;
            cmax = max(cmax, t + task.q);
            order.push(tasks.remove(idx));
        } else {
            t = tasks.iter().min_by(|a, b| a.r.cmp(&b.r)).unwrap().r;
        }
    }

    (order, cmax)
}

// vec not sorted because the sorting that restores invariant
// would have to happen in worst case n time which give the alg O(n^2logn)
pub fn schrage_preemptive_vecs_cmax(mut tasks: Vec<Task>) -> u32 {
    let mut t = 0;
    let mut cmax = 0;
    let mut current_task: Option<Task> = None;

    while !tasks.is_empty() {
        if let Some((idx, task)) = tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.r <= t)
            .max_by(|(_, a), (_, b)| a.q.cmp(&b.q))
        {
            let task = *task; // clone out of the vec so the references are valid
            if let Some(ref mut current) = current_task {
                if task.q > current.q {
                    current.p = t - task.r;
                    t = task.r;
                    if current.p > 0 {
                        tasks.push(*current);
                    }
                }
            }

            t += task.p;
            cmax = max(cmax, t + task.q);
            tasks.remove(idx);
            current_task = Some(task);
        } else {
            t = tasks.iter().min_by(|a, b| a.r.cmp(&b.r)).unwrap().r;
            current_task = None;
        }
    }

    cmax
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{correct_order, tasks};

    #[test]
    fn schrage_vecs_sort_q_test() {
        let tasks = tasks!();
        let cmax = schrage_vecs_sort_q_cmax(tasks);
        assert_eq!(cmax, 53);
    }

    #[test]
    fn schrage_vecs_sort_q_test_order() {
        let tasks = tasks!();
        let (order, cmax) = schrage_vecs_sort_q(tasks);
        assert_eq!(cmax, 53);
        assert_eq!(order, correct_order!());
    }

    #[test]
    fn schrage_vecs_v_sort_r_test() {
        let tasks = tasks!();
        let cmax = schrage_vecs_sort_r_cmax(tasks);
        assert_eq!(cmax, 53);
    }

    #[test]
    fn schrage_vecs_sort_r_test_order() {
        let tasks = tasks!();
        let (order, cmax) = schrage_vecs_sort_r(tasks);
        assert_eq!(cmax, 53);
        assert_eq!(order, correct_order!());
    }
}
