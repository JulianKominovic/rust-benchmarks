use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use rust_rm::rm_parallel;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("List files in directory");
    group.sample_size(100);
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function(
        "Parallel buffer first then push each line then stdout",
        |b| b.iter(|| rm_parallel::rm_parallel()),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
