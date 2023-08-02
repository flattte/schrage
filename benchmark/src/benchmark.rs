use core::ops::Range;
use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, PlotConfiguration, SamplingMode};
use lazy_static::lazy_static;
use rand::Rng;
use schrage::{custom_heap_impl::*, std_heap_impl::*, std_vecs_impl::*, task::Task};
use std::time::Duration;

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

fn gen_tasks(
    mutiplier_higher_bound: u32,
    step_by: usize,
    bound_r: u32,
    bound_p: u32,
    bound_q: u32,
) -> Vec<Vec<Task>> {
    (1..=mutiplier_higher_bound)
        .into_iter()
        .step_by(step_by)
        .map(|n| ((10 * n) as usize, (0..bound_r), (0..bound_p), (0..bound_q)))
        .map(|arg| gen_uniform(arg.0, &arg.1, &arg.2, &arg.3))
        .collect()
}

lazy_static! {
    #[derive(Debug)]
    static ref DATA: Vec<Vec<Task>> = gen_tasks(30, 3, 1000, 1000, 1000);
}

fn bench_algs_preemptive(c: &mut Criterion) {
    let sets_of_tasks: &Vec<Vec<Task>> = &*DATA;
    let mut group = c.benchmark_group("preemptive algs on random uniform data");
    group
        .sample_size(30)
        .sampling_mode(SamplingMode::Flat)
        .plot_config(PlotConfiguration::default())
        .warm_up_time(Duration::from_secs(1));

    for tasks in sets_of_tasks {
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_heaps_std_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_heaps_std_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_custom_heaps_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_custom_heaps_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_vecs_cmax", &tasks.len()),
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

    for tasks in sets_of_tasks {
        group.bench_with_input(
            BenchmarkId::new("schrage_heaps_std_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_heaps_std_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_custom_heaps_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_custom_heaps_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_vecs_sort_q_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_vecs_sort_q_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_vecs_sort_r_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_vecs_sort_r_cmax(input.clone())),
        );
    }
    group.finish();
}

lazy_static! {
    static ref DATA_FOR_HEAPS: Vec<Vec<Task>> = gen_tasks(10000, 1000, 100, 1000, 1000);
}

fn bench_on_big_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("heaps algs on lots of random uniform data");
    let sets_of_tasks = &*DATA_FOR_HEAPS;
    group
        .sampling_mode(SamplingMode::Flat)
        .plot_config(PlotConfiguration::default())
        .warm_up_time(Duration::from_secs(1));

    for tasks in sets_of_tasks {
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_heaps_std_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_heaps_std_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_custom_heaps_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_custom_heaps_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_heaps_std_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_heaps_std_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_custom_heaps_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_custom_heaps_cmax(input.clone())),
        );
    }
}

fn single_iter_through_long_data(c: &mut Criterion) {
    let sets_of_tasks: &Vec<Vec<Task>> =
        &vec![gen_uniform(2000_000, &(0..10000), &(0..10000), &(0..10000))];
    let mut group = c.benchmark_group("preemptive algs on random uniform data");
    group
        .sample_size(10)
        .sampling_mode(SamplingMode::Flat)
        .plot_config(PlotConfiguration::default())
        .warm_up_time(Duration::from_secs(1));
    for tasks in sets_of_tasks {
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_heaps_std_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_heaps_std_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_preemptive_custom_heaps_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_preemptive_custom_heaps_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_heaps_std_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_heaps_std_cmax(input.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("schrage_custom_heaps_cmax", &tasks.len()),
            tasks,
            |bencher, input| bencher.iter(|| schrage_custom_heaps_cmax(input.clone())),
        );
    }
}

criterion_group!(bench, bench_algs, bench_algs_preemptive, bench_on_big_data);
criterion_group!(bench_data_heaps, bench_on_big_data);
criterion_group!(bench_single_alot, single_iter_through_long_data);
criterion_main!(bench);
