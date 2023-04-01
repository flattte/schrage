#![allow(unused)]
use core::ops::Range;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration,
    SamplingMode,
};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::time::Duration;
mod mods;
use crate::mods::{heap_binary::*, std_heaps::*, std_vecs::*, task::Task};

fn gen_uniform(
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

fn gen_tasks() -> Vec<Vec<Task>> {
    (1..=50)
        .into_iter()
        .step_by(5)
        .map(|n| {
            (
                (1 * n) as usize,
                (0..100 * n / 2),
                (0..1000),
                (0..1000),
            )
        })
        .map(|arg| gen_uniform(arg.0, &arg.1, &arg.2, &arg.3))
        .collect()
}

lazy_static! {
    #[derive(Debug)]
    static ref DATA: Vec<Vec<Task>> = gen_tasks();
}

fn bench_algs_preemptive(c: &mut Criterion) {
    let sets_of_tasks: &Vec<Vec<Task>> = &*DATA;
    let mut group = c.benchmark_group("preemptive salgs on random uniform data");
    group
        .sample_size(30)
        .sampling_mode(SamplingMode::Flat)
        .plot_config(PlotConfiguration::default())
        .warm_up_time(Duration::from_secs(1));

    for (idx, tasks) in sets_of_tasks.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_heaps_std_cmax", &idx),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_heaps_std_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_custom_heaps_cmax", &idx),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_custom_heaps_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_vecs_cmax", &idx),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_vecs_cmax(input.clone())),
        );
    }
    group.finish();
}

fn bench_algs(c: &mut Criterion) {
    let sets_of_tasks: &Vec<Vec<Task>> = &*DATA;
    let mut group = c.benchmark_group("Non preemptive algs on random uniform data");
    group
        .sampling_mode(SamplingMode::Flat)
        .plot_config(PlotConfiguration::default())
        .warm_up_time(Duration::from_secs(1));

    for (idx, tasks) in sets_of_tasks.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("schrage_heaps_std_cmax", &idx),
            tasks,
            |bencher, input| bencher.iter(|| schrage_heaps_std_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_custom_heaps_cmax", &idx),
            tasks,
            |bencher, input| bencher.iter(|| schrage_custom_heaps_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_vecs_sort_q_cmax", &idx),
            tasks,
            |bencher, input| bencher.iter(|| schrage_vecs_sort_q_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_vecs_sort_r_cmax", &idx),
            tasks,
            |bencher, input| bencher.iter(|| schrage_vecs_sort_r_cmax(input.clone())),
        );
    }
    group.finish();
}

criterion_group!(bench, bench_algs, bench_algs_preemptive);
criterion_main!(bench);
