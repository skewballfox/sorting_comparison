use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use sorting_comparison::{
    hybrid_quick::hybrid_quick_sort, quick_sort::quick_sort, selection_sort::selection_sort,
};
//for generating random arrays of
use rand::{distributions::Uniform, Rng};

fn generate_random_vector(len: usize) -> Vec<i32> {
    let range = Uniform::from(0..1000);
    let values: Vec<i32> = rand::thread_rng().sample_iter(&range).take(len).collect();
    values
}
pub fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("crossover zoom in 2");

    let mut len;
    for i in 10..25 {
        len = i;

        group.bench_with_input(BenchmarkId::new("Quick Sort", len), &len, |b, len| {
            b.iter_batched(
                || generate_random_vector(*len).clone(),
                |mut data| quick_sort(&mut data),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("Selection Sort", len), &len, |b, len| {
            b.iter_batched(
                || generate_random_vector(*len).clone(),
                |mut data| selection_sort(&mut data),
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

pub fn bench_large_n(c: &mut Criterion) {
    let mut group = c.benchmark_group("hybrid test opt");

    let mut len;
    for i in 1..11 {
        len = i * 1000;

        group.bench_with_input(BenchmarkId::new("Quick Sort", len), &len, |b, len| {
            b.iter_batched(
                || generate_random_vector(*len).clone(),
                |mut data| quick_sort(&mut data),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(
            BenchmarkId::new("hybrid quick sort", len),
            &len,
            |b, len| {
                b.iter_batched(
                    || generate_random_vector(*len).clone(),
                    |mut data| hybrid_quick_sort(&mut data),
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_large_n);
criterion_main!(benches);
