# Schrage Algorithm

This projects aims to implement correct schrage algorithm
```sh
cargo test
```
and benchmark the rusults
```sh
cargo bench
```

## banchmark options

To change what benchmarks are run the src/benchmark.rs has to be changed. The lines that determine what benchmarks are run are atthe bottom of the file.

```rs
criterion_group!(bench, bench_algs, bench_algs_preemptive, bench_on_big_data);
criterion_group!(bench_data_heaps, bench_on_big_data);
criterion_group!(bench_single_alot, single_iter_through_long_data);
criterion_main!(bench);
```
the banchmarks are then located in target/criterion/report/index.html