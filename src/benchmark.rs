#![allow(unused)]
use core::ops::Range;
use rand::{thread_rng, Rng};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod mods;
use crate::mods::task::Task;


fn gen_gaussian(
    amount: usize,
    r_vals: &Range<u32>,
    p_vals: &Range<u32>,
    q_vals: &Range<u32>,
) -> Vec<Task> {
    let mut rng = rand::thread_rng();
    let mut tasks = Vec::new();
    for _ in 0..amount {
        let r = rng.gen_range(r_vals.to_owned());
        let p = rng.gen_range(p_vals.to_owned());
        let q = rng.gen_range(q_vals.to_owned());
        tasks.push(Task { r, p, q });
    }
    tasks
}



fn bench_alg_on_random_time() -> usize {
    let tasks = gen_gaussian(10000, &(0..10000), &(0..10000), &(0..10000));
    
    0
}
fn benches() {}

//criterion_group!(benches, );
criterion_main!(benches);