use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use rust_ls::{
    buffer_first_then_stdout, buffer_reserve_then_stdout, directly_stdout,
    directly_stdout_manual_std_lock, parallel_buffer_first_then_stdout,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("List files in directory");
    group.sample_size(100);
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function(
        "Parallel buffer first then push each line then stdout",
        |b| b.iter(|| parallel_buffer_first_then_stdout::buffer_first_then_stdout()),
    );
    group.bench_function("Buffer first then push each line then stdout", |b| {
        b.iter(|| buffer_first_then_stdout::buffer_first_then_stdout())
    });
    group.bench_function("Buffer first reserving massive memory (to avoid allocation) then push each line then stdout", |b| {
        b.iter(|| buffer_reserve_then_stdout::buffer_reserve_then_stdout())
    });
    group.bench_function("Stdout directly", |b| {
        b.iter(|| directly_stdout::directly_stdout())
    });
    group.bench_function("Stdout directly manual std lock", |b| {
        b.iter(|| directly_stdout_manual_std_lock::directly_stdout_manual_stdout_lock())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
